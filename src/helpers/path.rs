use anyhow::{anyhow, Result};
use homedir::my_home;
use std::path::PathBuf;

pub fn home() -> Result<PathBuf> {
  Ok(my_home()?.ok_or(anyhow!("获取用户缓存目录失败"))?)
}

pub fn cache() -> Result<PathBuf> {
  Ok(home()?.join(".WuKong"))
}
