use crate::types::Mode;
use anyhow::{anyhow, Result};
use parking_lot::RwLock;
use rocksdb::{Direction, IteratorMode, Options, WriteBatch, DB};
use std::{
  ops::{Deref, DerefMut},
  sync::Arc,
};

pub trait Database: Send + Sync {
  fn get<T, K, V>(&self, table: T, key: K) -> Result<Option<V>>
  where
    T: AsRef<str>,
    K: bincode::Encode,
    V: bincode::Decode;

  fn get_range<T, K, V1, V2, I>(&self, table: T, begin: K, end: K) -> Result<Vec<(V1, V2)>>
  where
    T: AsRef<str>,
    K: bincode::Encode,
    V1: bincode::Decode,
    V2: bincode::Decode;

  fn set<T, K, V>(&self, table: T, key: K, val: V) -> Result<()>
  where
    T: AsRef<str>,
    K: bincode::Encode,
    V: bincode::Encode;

  fn batch_set<T, K, V, I>(&self, table: T, iter: I) -> Result<()>
  where
    T: AsRef<str>,
    K: bincode::Encode,
    V: bincode::Encode,
    I: Iterator<Item = (K, V)>;
}

pub struct DatabaseProvider(Arc<RwLock<DB>>);

impl Deref for DatabaseProvider {
  type Target = Arc<RwLock<DB>>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for DatabaseProvider {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl Database for DatabaseProvider {
  fn get<T, K, V>(&self, table: T, key: K) -> Result<Option<V>>
  where
    T: AsRef<str>,
    K: bincode::Encode,
    V: bincode::Decode,
  {
    self.init_table(&table)?;
    let db = self.read();
    let table = db.cf_handle(table.as_ref()).ok_or(anyhow!("table not found"))?;
    let key = bincode::encode_to_vec(key, bincode::config::standard().with_big_endian())?;
    let val = db.get_cf(table, key)?;
    match val {
      Some(v) => {
        let (val, _) = bincode::decode_from_slice(&v, bincode::config::standard())?;
        Ok(Some(val))
      },
      None => Ok(None),
    }
  }

  fn get_range<T, K, V1, V2, I>(&self, table: T, begin: K, end: K) -> Result<Vec<(V1, V2)>>
  where
    T: AsRef<str>,
    K: bincode::Encode,
    V1: bincode::Decode,
    V2: bincode::Decode,
  {
    self.init_table(&table)?;
    let db = self.read();
    let table = db.cf_handle(table.as_ref()).ok_or(anyhow!("table not found"))?;
    let begin = bincode::encode_to_vec(begin, bincode::config::standard().with_big_endian())?;
    let end = bincode::encode_to_vec(end, bincode::config::standard().with_big_endian())?;
    let iter = db.iterator_cf(table, IteratorMode::From(&begin, Direction::Forward));

    let mut items = vec![];
    for item in iter {
      let (key, val) = item?;

      if key.as_ref().gt(end.as_slice()) {
        break;
      }

      let (key, _) =
        bincode::decode_from_slice(&key, bincode::config::standard().with_big_endian())?;
      let (val, _) = bincode::decode_from_slice(&val, bincode::config::standard())?;

      items.push((key, val));
    }
    Ok(items)
  }

  fn set<T, K, V>(&self, table: T, key: K, val: V) -> Result<()>
  where
    T: AsRef<str>,
    K: bincode::Encode,
    V: bincode::Encode,
  {
    self.init_table(&table)?;
    let db = self.read();
    let table = db.cf_handle(table.as_ref()).ok_or(anyhow!("table not found"))?;
    let key = bincode::encode_to_vec(key, bincode::config::standard().with_big_endian())?;
    let val = bincode::encode_to_vec(val, bincode::config::standard())?;

    db.put_cf(table, key, val)?;

    Ok(())
  }

  fn batch_set<T, K, V, I>(&self, table: T, iter: I) -> Result<()>
  where
    T: AsRef<str>,
    K: bincode::Encode,
    V: bincode::Encode,
    I: Iterator<Item = (K, V)>,
  {
    self.init_table(&table)?;
    let db = self.read();
    let table = db.cf_handle(table.as_ref()).ok_or(anyhow!("table not found"))?;

    let mut batch = WriteBatch::default();

    for (key, val) in iter {
      let key = bincode::encode_to_vec(key, bincode::config::standard().with_big_endian())?;
      let val = bincode::encode_to_vec(val, bincode::config::standard())?;
      batch.put_cf(table, key, val);
    }

    db.write(batch)?;

    Ok(())
  }
}

impl DatabaseProvider {
  fn init_table<T>(&self, table: T) -> Result<()>
  where
    T: AsRef<str>,
  {
    let mut db = self.write();
    if db.cf_handle(table.as_ref()).is_none() {
      db.create_cf(table.as_ref(), &Options::default())?;
    }
    Ok(())
  }
}

pub fn open(mode: Mode) -> Result<DatabaseProvider> {
  let path = crate::helpers::path::cache()?.join(mode.as_ref());
  let mut opts = Options::default();
  opts.create_if_missing(true);
  let cfs = if path.exists() { DB::list_cf(&opts, &path)? } else { vec![] };
  let db = DB::open_cf(&opts, path, cfs)?;
  Ok(DatabaseProvider(Arc::new(RwLock::new(db))))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tests() -> Result<()> {
    open(Mode::Backtest)?;
    Ok(())
  }
}
