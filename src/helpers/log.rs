use super::constants::Environment;
use anyhow::Result;
use std::str::FromStr;
use time::{macros::format_description, UtcOffset};
// use tracing::subscriber::DefaultGuard;
use tracing_subscriber::{filter::LevelFilter, fmt::time::OffsetTime, layer::SubscriberExt};

pub fn init() -> Result<()> {
  let level =
    LevelFilter::from_str(Environment::LogLevel.value().unwrap_or("debug".into()).as_str())?;

  let color = Environment::LogColor.value().unwrap_or("true".to_owned()).to_lowercase() == "true";

  let time_fmt = if Environment::LogMs.value().unwrap_or_default().to_lowercase() == "true" {
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]")
  } else {
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second]")
  };

  let local_time =
    OffsetTime::new(UtcOffset::from_hms(8, 0, 0).expect("time zone conversion failed"), time_fmt);

  let sub = tracing_subscriber::fmt()
    .with_max_level(level)
    .with_ansi(color)
    .with_level(true)
    .with_file(false)
    .with_thread_ids(false)
    .with_thread_names(false)
    .with_target(false)
    .with_line_number(false)
    .with_timer(local_time)
    .finish();

  let sub =
    sub.with(tracing_subscriber::filter::Targets::new().with_target("wukong", LevelFilter::TRACE));

  // let _guard = tracing::subscriber::set_default(sub);

  tracing::subscriber::set_global_default(sub)?;

  Ok(())
}

#[cfg(test)]
mod tests {
  use crate::helpers::constants::Environment;
  use anyhow::Result;

  #[test]
  fn tests() -> Result<()> {
    Environment::LogColor.set_value("true");
    let _guard = super::init()?;
    tracing::info!("123");
    Ok(())
  }
}
