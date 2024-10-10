use crate::types::{Candle, Mode, TimeFrame};
use anyhow::{bail, Result};
use bincode::{decode_from_slice, encode_to_vec, Decode, Encode};
use redb::{Key, TableDefinition, TableError, Value};
use std::{cmp::Ordering, fmt::Debug, ops::Deref, sync::Arc};

#[derive(Debug)]
pub struct Bincode<T>(pub T);

impl<T> Value for Bincode<T>
where
  T: Debug + Encode + Decode,
{
  type SelfType<'a> = T
    where
        Self: 'a;

  type AsBytes<'a> = Vec<u8>
    where
        Self: 'a;

  fn fixed_width() -> Option<usize> {
    None
  }

  fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
  where
    Self: 'a,
  {
    let (decoded, _) =
      decode_from_slice(&data, bincode::config::standard().with_big_endian()).unwrap();
    decoded
  }

  fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
  where
    Self: 'a,
    Self: 'b,
  {
    encode_to_vec(value, bincode::config::standard().with_big_endian()).unwrap()
  }

  fn type_name() -> redb::TypeName {
    redb::TypeName::new(&format!("Bincode<{}>", std::any::type_name::<T>()))
  }
}

impl<T> Key for Bincode<T>
where
  T: Debug + Encode + Decode + Ord,
{
  fn compare(data1: &[u8], data2: &[u8]) -> Ordering {
    Self::from_bytes(data1).cmp(&Self::from_bytes(data2))
  }
}

pub trait DatabaseAPI: Send + Sync {
  ////////////////////////////////////////////////////////////////////////////////////////
  // 行情
  ////////////////////////////////////////////////////////////////////////////////////////

  /// 获取K线
  fn get_candle(
    &self,
    mode: Mode,
    symbol: &str,
    interval: TimeFrame,
    time: i64,
  ) -> Result<Option<Candle>>;
  /// 获取K线列表
  fn get_candles(
    &self,
    mode: Mode,
    symbol: &str,
    interval: TimeFrame,
    begin: i64,
    end: i64,
  ) -> Result<Vec<Candle>>;
  /// 保存K线
  fn save_candles(
    &self,
    mode: Mode,
    symbol: &str,
    interval: TimeFrame,
    candles: &Vec<Candle>,
  ) -> Result<()>;

  ////////////////////////////////////////////////////////////////////////////////////////
  // 回测
  ////////////////////////////////////////////////////////////////////////////////////////

  /// 获取回测K线配置
  fn get_backtest_candle_config(&self, symbol: &str, interval: TimeFrame) -> Result<Option<i64>>;
  /// 保存回测K线配置
  fn save_backtest_candle_config(&self, symbol: &str, interval: TimeFrame, time: i64)
    -> Result<()>;
}

pub type Database = Arc<dyn DatabaseAPI>;

/// 打开数据库
pub fn open(mode: Mode) -> Result<Database> {
  let dir = super::path::cache()?;
  let path = dir.join(mode.as_ref()).with_extension("db");
  let db = if path.exists() {
    redb::Database::open(path)?
  } else {
    std::fs::create_dir_all(&dir)?;
    redb::Database::create(path)?
  };
  Ok(Arc::new(DatabaseProvider(db)))
}

struct DatabaseProvider(redb::Database);

impl Deref for DatabaseProvider {
  type Target = redb::Database;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatabaseAPI for DatabaseProvider {
  fn get_candle(
    &self,
    mode: Mode,
    symbol: &str,
    interval: TimeFrame,
    time: i64,
  ) -> Result<Option<Candle>> {
    let tx = self.begin_read()?;
    let table = format!("{}_{}_{}", mode.as_ref(), symbol, interval);
    let table: redb::TableDefinition<Bincode<i64>, Bincode<Candle>> = TableDefinition::new(&table);
    let table = match tx.open_table(table) {
      Ok(table) => table,
      Err(err) =>
        if matches!(err, TableError::TableDoesNotExist(_)) {
          return Ok(None)
        } else {
          bail!(err)
        },
    };
    let value = table.get(time)?;
    Ok(value.map(|v| v.value()))
  }

  fn get_candles(
    &self,
    mode: Mode,
    symbol: &str,
    interval: TimeFrame,
    begin: i64,
    end: i64,
  ) -> Result<Vec<Candle>> {
    let tx = self.begin_read()?;
    let table = format!("{}_{}_{}", mode.as_ref(), symbol, interval);
    let table: redb::TableDefinition<Bincode<i64>, Bincode<Candle>> = TableDefinition::new(&table);
    let table = match tx.open_table(table) {
      Ok(table) => table,
      Err(err) =>
        if matches!(err, TableError::TableDoesNotExist(_)) {
          return Ok(vec![])
        } else {
          bail!(err)
        },
    };
    let mut candles = vec![];
    for value in table.range(begin..=end)? {
      let (_, value) = value?;
      candles.push(value.value());
    }
    Ok(candles)
  }

  fn save_candles(
    &self,
    mode: Mode,
    symbol: &str,
    interval: TimeFrame,
    candles: &Vec<Candle>,
  ) -> Result<()> {
    let tx = self.begin_write()?;
    {
      let table = format!("{}_{}_{}", mode.as_ref(), symbol, interval);
      let table: redb::TableDefinition<Bincode<i64>, Bincode<Candle>> =
        TableDefinition::new(&table);
      let mut table = tx.open_table(table)?;
      for candle in candles {
        table.insert(candle.time, candle)?;
      }
    }
    tx.commit()?;
    Ok(())
  }

  fn get_backtest_candle_config(&self, symbol: &str, interval: TimeFrame) -> Result<Option<i64>> {
    let tx = self.begin_read()?;
    let table = format!("{}_Candle_Config", Mode::Backtest.as_ref());
    let table: redb::TableDefinition<Bincode<String>, Bincode<i64>> = TableDefinition::new(&table);
    let table = match tx.open_table(table) {
      Ok(table) => table,
      Err(err) =>
        if matches!(err, TableError::TableDoesNotExist(_)) {
          return Ok(None)
        } else {
          bail!(err)
        },
    };
    let key = format!("{}_{}", symbol, interval.as_ref());
    let value = table.get(key)?;
    Ok(value.map(|v| v.value()))
  }

  fn save_backtest_candle_config(
    &self,
    symbol: &str,
    interval: TimeFrame,
    time: i64,
  ) -> Result<()> {
    let tx = self.begin_write()?;
    {
      let table = format!("{}_Candle_Config", Mode::Backtest.as_ref());
      let table: redb::TableDefinition<Bincode<String>, Bincode<i64>> =
        TableDefinition::new(&table);
      let mut table = tx.open_table(table)?;
      let key = format!("{}_{}", symbol, interval.as_ref());
      table.insert(key, time)?;
    }
    tx.commit()?;
    Ok(())
  }
}

#[cfg(test)]
#[allow(unused)]
mod tests {
  use super::*;

  #[test]
  fn test_open_db() -> Result<()> {
    let db = super::open(Mode::Backtest)?;
    // let candle = db
    //     .lock()
    //     .get_candle(Mode::Backtest, "BTCUSDT", CandleInterval::Day, 0)?;
    // println!("{:?}", candle);
    // let candles = vec![Candle {
    //     time: 0,
    //     open: 0.0,
    //     high: 0.0,
    //     low: 0.0,
    //     close: 0.0,
    //     volume: 0.0,
    //     amount: 0.0,
    //     taker_volume: 0.0,
    //     taker_amount: 0.0,
    //     trades: 0,
    // }];
    // db.lock()
    //     .save_candles(Mode::Backtest, "BTCUSDT", CandleInterval::Day, &candles)?;
    // db.lock()
    //     .save_backtest_candle_config("BTCUSDT", CandleInterval::Minute, 123)?;
    // let v = db
    //     .lock()
    //     .get_backtest_candle_config("BTCUSDT", CandleInterval::Minute)?;
    // println!("{:?}", v);
    Ok(())
  }
}
