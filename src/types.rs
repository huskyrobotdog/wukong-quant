use chrono::Duration;
use pyo3::prelude::*;
use strum::{AsRefStr, Display};

/// 运行模式
#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Display, AsRefStr)]
pub enum Mode {
  /// 回测
  Backtest,
  /// 模拟
  Sandbox,
  /// 实盘
  Real,
}

/// 交易类型
#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Display, AsRefStr)]
pub enum Type {
  /// 限价交易
  Limit,
  /// 市价交易
  Market,
}

/// 交易方向
#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Display, AsRefStr)]
pub enum Side {
  /// 做多
  Long,
  /// 做空
  Short,
}

/// 时间周期
#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Display, AsRefStr)]
pub enum TimeFrame {
  #[strum(serialize = "1m")]
  Minute,
  #[strum(serialize = "3m")]
  Minute3,
  #[strum(serialize = "5m")]
  Minute5,
  #[strum(serialize = "15m")]
  Minute15,
  #[strum(serialize = "30m")]
  Minute30,
  #[strum(serialize = "1h")]
  Hour,
  #[strum(serialize = "2h")]
  Hour2,
  #[strum(serialize = "4h")]
  Hour4,
  #[strum(serialize = "6h")]
  Hour6,
  #[strum(serialize = "8h")]
  Hour8,
  #[strum(serialize = "12h")]
  Hour12,
  #[strum(serialize = "1d")]
  Day,
  #[strum(serialize = "3d")]
  Day3,
  #[strum(serialize = "1w")]
  Week,
  #[strum(serialize = "1M")]
  Month,
}

impl TimeFrame {
  pub fn as_duration(&self) -> Duration {
    match self {
      Self::Minute => Duration::minutes(1),
      Self::Minute3 => Duration::minutes(3),
      Self::Minute5 => Duration::minutes(5),
      Self::Minute15 => Duration::minutes(15),
      Self::Minute30 => Duration::minutes(30),
      Self::Hour => Duration::hours(1),
      Self::Hour2 => Duration::hours(2),
      Self::Hour4 => Duration::hours(4),
      Self::Hour6 => Duration::hours(6),
      Self::Hour8 => Duration::hours(8),
      Self::Hour12 => Duration::hours(12),
      Self::Day => Duration::days(1),
      Self::Day3 => Duration::days(3),
      Self::Week => Duration::weeks(1),
      Self::Month => Duration::weeks(4),
    }
  }
}

/// 订单状态
#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Display, AsRefStr)]
pub enum OrderStatus {
  /// 创建
  Created,
  /// 已提交
  Submited,
  /// 挂单中
  Pending,
  /// 部分成交
  Partial,
  /// 完全成交
  Completed,
  /// 被拒绝
  Rejected,
  /// 已取消
  Canceled,
}
