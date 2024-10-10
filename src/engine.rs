use crate::{
  global::{set_engine, set_strategy_callback, strategy_callback},
  helpers::constants::{Environment, BANNER},
  models::{Context, StrategyCallback},
  types::Mode,
};
use anyhow::Result;
use parking_lot::Mutex;
use std::{
  ops::{Deref, DerefMut},
  sync::Arc,
};

pub struct Engine(Context);

impl Deref for Engine {
  type Target = Context;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Engine {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

pub fn start(mode: Mode, strategy: &str) -> Result<()> {
  // 显示Banner
  if Environment::ShowBanner.as_bool(true) {
    println!("{}", BANNER);
  }

  // 初始化日志
  crate::helpers::log::init()?;

  // 初始化rayon
  crate::helpers::runtime::init_rayon()?;

  // 上下文
  let ctx = Context {
    db: crate::helpers::database::open(mode)?,
    running: false,
    trade_time: Default::default(),
    benchmark: Default::default(),
    account: Default::default(),
    pairs: Default::default(),
  };

  // 引擎
  set_engine(Arc::new(Mutex::new(Engine(ctx))))?;

  // 策略回调
  set_strategy_callback(Arc::new(StrategyCallback::new(strategy)?))?;

  // 策略初始化
  strategy_callback().on_init()?;

  // 策略停止运行
  strategy_callback().on_stop()?;

  Ok(())
}
