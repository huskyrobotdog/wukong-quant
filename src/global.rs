use crate::{engine::Engine, models::StrategyCallback};
use anyhow::{anyhow, Result};
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use std::sync::Arc;

static ENGINE: OnceCell<Arc<Mutex<Engine>>> = OnceCell::new();

pub fn set_engine(engine: Arc<Mutex<Engine>>) -> Result<()> {
  ENGINE.set(engine).map_err(|_| anyhow!("设置引擎失败"))?;
  Ok(())
}

pub fn engine() -> Arc<Mutex<Engine>> {
  ENGINE.get().cloned().unwrap()
}

static STRATEGY_CALLBACK: OnceCell<Arc<StrategyCallback>> = OnceCell::new();

pub fn set_strategy_callback(sc: Arc<StrategyCallback>) -> Result<()> {
  STRATEGY_CALLBACK.set(sc).map_err(|_| anyhow!("设置策略回调失败"))?;
  Ok(())
}

pub fn strategy_callback() -> Arc<StrategyCallback> {
  STRATEGY_CALLBACK.get().cloned().unwrap()
}
