use anyhow::{anyhow, Result};
use chrono::{DateTime, TimeZone, Utc};

/// 字符串转日期 `UTC+0`
/// ---
/// - 格式1 : 2000
/// - 格式2 : 200001
/// - 格式3 : 20000102
/// - 格式4 : 2000010203
/// - 格式5 : 200001020304
/// - 格式6 : 20000102030405
/// ---
/// 其他格式均返回错误
pub fn str_to_date(s: &str) -> Result<DateTime<Utc>> {
  let t = if s.len() == 4 {
    DateTime::parse_from_str(&format!("{s}0101000000+00:00"), "%Y%m%d%H%M%S%z")?.to_utc()
  } else if s.len() == 6 {
    DateTime::parse_from_str(&format!("{s}01000000+00:00"), "%Y%m%d%H%M%S%z")?.to_utc()
  } else if s.len() == 8 {
    DateTime::parse_from_str(&format!("{s}000000+00:00"), "%Y%m%d%H%M%S%z")?.to_utc()
  } else if s.len() == 10 {
    DateTime::parse_from_str(&format!("{s}0000+00:00"), "%Y%m%d%H%M%S%z")?.to_utc()
  } else if s.len() == 12 {
    DateTime::parse_from_str(&format!("{s}00+00:00"), "%Y%m%d%H%M%S%z")?.to_utc()
  } else if s.len() == 14 {
    DateTime::parse_from_str(&format!("{s}+00:00"), "%Y%m%d%H%M%S%z")?.to_utc()
  } else {
    return Err(anyhow!("wrong time format : {}", s));
  };
  Ok(t)
}

/// 毫秒转日期 `UTC+0`
pub fn ms_to_date(ts: i64) -> Result<DateTime<Utc>> {
  Utc.timestamp_millis_opt(ts).single().ok_or(anyhow!("时间戳转换失败"))
}

/// 当前毫秒时间戳
pub fn now_ms() -> i64 {
  Utc::now().timestamp_millis()
}

#[cfg(test)]
mod tests {
  use anyhow::Result;

  #[test]
  fn tests() -> Result<()> {
    println!("{}", super::str_to_date("2024")?);
    println!("{}", super::str_to_date("202402")?);
    println!("{}", super::str_to_date("20240203")?);
    println!("{}", super::str_to_date("2024020304")?);
    println!("{}", super::str_to_date("202402030405")?);
    println!("{}", super::str_to_date("20240203040506")?);
    Ok(())
  }
}
