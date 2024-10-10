use crate::{models::Candle, types::TimeFrame};
use anyhow::Result;

pub trait Database: Send + Sync {
  /// 获取K线
  fn get_candle(&self, symbol: &str, interval: TimeFrame, time: i64) -> Result<Option<Candle>>;
  /// 获取K线列表
  fn get_candles(
    &self,
    symbol: &str,
    interval: TimeFrame,
    begin: i64,
    end: i64,
  ) -> Result<Vec<Candle>>;
  /// 保存K线
  fn save_candles(&self, symbol: &str, interval: TimeFrame, candles: &Vec<Candle>) -> Result<()>;
  /// 获取回测K线配置
  fn get_backtest_candle_config(&self, symbol: &str, interval: TimeFrame) -> Result<Option<i64>>;
  /// 保存回测K线配置
  fn save_backtest_candle_config(&self, symbol: &str, interval: TimeFrame, time: i64)
    -> Result<()>;
}

// /// 打开数据库
// pub fn open(mode: Mode) -> Result<Database> {
//   let dir = super::path::cache()?;
//   let path = dir.join(mode.as_ref()).with_extension("db");
//   let db = if path.exists() {
//     redb::Database::open(path)?
//   } else {
//     std::fs::create_dir_all(&dir)?;
//     redb::Database::create(path)?
//   };
//   Ok(Arc::new(DatabaseProvider(db)))
// }
