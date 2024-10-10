use anyhow::Result;

pub trait Database: Send + Sync {
  fn get<T, K, V>(&self, table: T, key: K) -> Result<Option<V>>
  where
    T: AsRef<str>,
    K: bincode::Encode,
    V: bincode::Decode;

  fn get_range<T, K, V, I>(&self, table: T, begin: K, end: K) -> Result<I>
  where
    T: AsRef<str>,
    K: bincode::Encode,
    V: bincode::Decode,
    I: Iterator<Item = (K, V)>;

  fn set<T, K, V>(&self, table: T, key: K, val: V) -> Result<()>
  where
    T: AsRef<str>,
    K: bincode::Encode,
    V: bincode::Decode;

  fn batch_set<T, K, V, I>(&self, table: T, iter: I) -> Result<()>
  where
    T: AsRef<str>,
    K: bincode::Encode,
    V: bincode::Decode,
    I: Iterator<Item = (K, V)>;
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
