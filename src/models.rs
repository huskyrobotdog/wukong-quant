use crate::types::{OrderStatus, Side, Type};
use anyhow::{bail, Result};
use bincode::{Decode, Encode};
use chrono::prelude::*;
use pyo3::prelude::*;
use rust_decimal::Decimal;

/// 账户
#[derive(Debug, Default, Clone)]
pub struct Account {
  /// 资金
  pub cash: Decimal,
  /// 可用资金
  pub avail_cash: Decimal,
  /// 保证金
  pub margin: Decimal,
  /// 仓位未实现盈亏
  pub pos_pnl: Decimal,
}

/// 仓位
#[derive(Debug, Clone)]
pub struct Position {
  /// 交易对
  pub symbol: String,
  /// 方向
  pub side: Side,
  /// 杠杆倍数
  pub leverage: Decimal,
  /// 标记价格
  pub mark_price: Decimal,
  /// 持仓
  pub size: Decimal,
  /// 可用持仓
  pub avail_size: Decimal,
  /// 均价
  pub price: Decimal,
  /// 保证金
  pub margin: Decimal,
  /// 未实现盈亏
  pub pnl: Decimal,
}

/// 订单
#[pyclass(get_all)]
#[derive(Debug, Clone)]
pub struct Order {
  /// 交易对
  pub symbol: String,
  /// 订单ID
  pub id: String,
  /// 类型
  pub r#type: Type,
  /// 方向
  pub side: Side,
  /// 减仓
  pub reduce: bool,
  /// 杠杆倍数
  pub leverage: Decimal,
  /// 数量
  pub size: Decimal,
  /// 价格
  pub price: Decimal,
  /// 下单时间
  pub time: DateTime<Utc>,
  /// 保证金
  pub margin: Decimal,
  /// 成交数量
  pub deal_size: Decimal,
  /// 成交均价
  pub deal_price: Decimal,
  /// 成交手续费
  pub deal_fee: Decimal,
  /// 状态
  pub status: OrderStatus,
}

/// K线
#[derive(Debug, Clone, Encode, Decode)]
pub struct Candle {
  /// 开盘时间
  pub time: i64,
  /// 开盘价
  pub open: f64,
  /// 最高价
  pub high: f64,
  /// 最低价
  pub low: f64,
  /// 收盘价
  pub close: f64,
  /// 数量
  pub volume: f64,
  /// 金额
  pub amount: f64,
  /// 吃单数量
  pub taker_volume: f64,
  /// 吃单金额
  pub taker_amount: f64,
  /// 成交笔数
  pub trades: i64,
}

/// 策略事件
pub struct StrategyEvent {
  /// 初始化
  on_init: Option<Py<PyAny>>,
  /// 每日开始
  on_day_begin: Option<Py<PyAny>>,
  /// 每小时开始
  on_hour_begin: Option<Py<PyAny>>,
  /// 每分钟开始
  on_minute_begin: Option<Py<PyAny>>,
  /// tick
  on_tick: Option<Py<PyAny>>,
  /// 每分钟结束
  on_minute_end: Option<Py<PyAny>>,
  /// 每小时结束
  on_hour_end: Option<Py<PyAny>>,
  /// 每天结束
  on_day_end: Option<Py<PyAny>>,
  /// 停止运行
  on_stop: Option<Py<PyAny>>,
}

impl StrategyEvent {
  pub fn new(strategy: &str) -> Result<Self> {
    Python::with_gil(|py| {
      let code = std::fs::read_to_string(strategy)?;
      let module =
        PyModule::from_code_bound(py, &code, strategy, format!("_{}", helpers::gen_id()).as_str())?;
      anyhow::Ok(Self {
        on_init: Self::get_call(&module, "on_initialize")?,
        on_day_begin: Self::get_call(&module, "on_day_begin")?,
        on_hour_begin: Self::get_call(&module, "on_hour_begin")?,
        on_minute_begin: Self::get_call(&module, "on_minute_begin")?,
        on_tick: Self::get_call(&module, "on_tick")?,
        on_minute_end: Self::get_call(&module, "on_minute_end")?,
        on_hour_end: Self::get_call(&module, "on_hour_end")?,
        on_day_end: Self::get_call(&module, "on_day_end")?,
        on_stop: Self::get_call(&module, "on_stop")?,
      })
    })
  }

  fn get_call(module: &Bound<'_, PyModule>, method: &str) -> Result<Option<Py<PyAny>>> {
    if module.hasattr(method)? {
      Ok(Some(module.getattr(method)?.unbind()))
    } else {
      Ok(None)
    }
  }
}

impl StrategyEvent {
  pub fn on_init(&self) -> Result<()> {
    if let Some(call) = &self.on_init {
      Python::with_gil(|py| {
        let call = call.bind(py);
        call.call0()?;
        anyhow::Ok(())
      })?;
      return Ok(());
    } else {
      bail!("未定义初始化事件");
    }
  }
  pub fn on_day_begin(&self) -> Result<()> {
    if let Some(call) = &self.on_day_begin {
      Python::with_gil(|py| {
        let call = call.bind(py);
        call.call0()?;
        anyhow::Ok(())
      })?;
      return Ok(());
    }
    Ok(())
  }
  pub fn on_hour_begin(&self) -> Result<()> {
    if let Some(call) = &self.on_hour_begin {
      Python::with_gil(|py| {
        let call = call.bind(py);
        call.call0()?;
        anyhow::Ok(())
      })?;
      return Ok(());
    }
    Ok(())
  }
  pub fn on_minute_begin(&self) -> Result<()> {
    if let Some(call) = &self.on_minute_begin {
      Python::with_gil(|py| {
        let call = call.bind(py);
        call.call0()?;
        anyhow::Ok(())
      })?;
      return Ok(());
    }
    Ok(())
  }
  pub fn on_tick(&self) -> Result<()> {
    if let Some(call) = &self.on_tick {
      Python::with_gil(|py| {
        let call = call.bind(py);
        call.call0()?;
        anyhow::Ok(())
      })?;
      return Ok(());
    }
    Ok(())
  }
  pub fn on_minute_end(&self) -> Result<()> {
    if let Some(call) = &self.on_minute_end {
      Python::with_gil(|py| {
        let call = call.bind(py);
        call.call0()?;
        anyhow::Ok(())
      })?;
      return Ok(());
    }
    Ok(())
  }
  pub fn on_hour_end(&self) -> Result<()> {
    if let Some(call) = &self.on_hour_end {
      Python::with_gil(|py| {
        let call = call.bind(py);
        call.call0()?;
        anyhow::Ok(())
      })?;
      return Ok(());
    }
    Ok(())
  }
  pub fn on_day_end(&self) -> Result<()> {
    if let Some(call) = &self.on_day_end {
      Python::with_gil(|py| {
        let call = call.bind(py);
        call.call0()?;
        anyhow::Ok(())
      })?;
      return Ok(());
    }
    Ok(())
  }

  pub fn on_stop(&self) -> Result<()> {
    if let Some(call) = &self.on_stop {
      Python::with_gil(|py| {
        let call = call.bind(py);
        call.call0()?;
        anyhow::Ok(())
      })?;
      return Ok(());
    }
    Ok(())
  }
}
