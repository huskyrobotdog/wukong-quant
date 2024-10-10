use crate::{
  global,
  models::Order,
  types::{Mode, OrderStatus},
};
use anyhow::Result;
use chrono::{DateTime, Utc};
use pyo3::{pyfunction, Python};
use rust_decimal::Decimal;

#[pyfunction]
#[pyo3(signature = (s))]
pub fn str_to_date(s: &str) -> Result<DateTime<Utc>> {
  crate::helpers::date::str_to_date(s)
}

#[pyfunction]
#[pyo3(signature = (ts))]
pub fn ms_to_date(ts: i64) -> Result<DateTime<Utc>> {
  crate::helpers::date::ms_to_date(ts)
}

#[pyfunction]
#[pyo3(signature = ())]
pub fn now_ms() -> i64 {
  crate::helpers::date::now_ms()
}

#[pyfunction]
#[pyo3(signature = ())]
pub fn rand_id() -> String {
  crate::helpers::id::gen()
}

#[pyfunction]
#[pyo3(signature = ())]
pub fn account_cash() -> Decimal {
  global::engine().lock().account.cash
}

#[pyfunction]
#[pyo3(signature = ())]
pub fn account_available_cash() -> Decimal {
  global::engine().lock().account.available_cash
}

#[pyfunction]
#[pyo3(signature = ())]
pub fn account_margin() -> Decimal {
  global::engine().lock().account.margin
}

#[pyfunction]
#[pyo3(signature = ())]
pub fn account_pnl() -> Decimal {
  global::engine().lock().account.pnl
}

#[pyfunction]
#[pyo3(signature = (symbol))]
pub fn position_long_size(symbol: &str) -> Option<Decimal> {
  global::engine().lock().pairs.get(symbol).map(|v| v.long.size)
}

#[pyfunction]
#[pyo3(signature = (symbol))]
pub fn position_long_available_size(symbol: &str) -> Option<Decimal> {
  global::engine().lock().pairs.get(symbol).map(|v| v.long.available_size)
}

#[pyfunction]
#[pyo3(signature = (symbol))]
pub fn position_long_price(symbol: &str) -> Option<Decimal> {
  global::engine().lock().pairs.get(symbol).map(|v| v.long.price)
}

#[pyfunction]
#[pyo3(signature = (symbol))]
pub fn position_long_margin(symbol: &str) -> Option<Decimal> {
  global::engine().lock().pairs.get(symbol).map(|v| v.long.margin)
}

#[pyfunction]
#[pyo3(signature = (symbol))]
pub fn position_long_pnl(symbol: &str) -> Option<Decimal> {
  global::engine().lock().pairs.get(symbol).map(|v| v.long.pnl)
}

#[pyfunction]
#[pyo3(signature = (symbol))]
pub fn position_short_size(symbol: &str) -> Option<Decimal> {
  global::engine().lock().pairs.get(symbol).map(|v| v.short.size)
}

#[pyfunction]
#[pyo3(signature = (symbol))]
pub fn position_short_available_size(symbol: &str) -> Option<Decimal> {
  global::engine().lock().pairs.get(symbol).map(|v| v.short.available_size)
}

#[pyfunction]
#[pyo3(signature = (symbol))]
pub fn position_short_price(symbol: &str) -> Option<Decimal> {
  global::engine().lock().pairs.get(symbol).map(|v| v.short.price)
}

#[pyfunction]
#[pyo3(signature = (symbol))]
pub fn position_short_margin(symbol: &str) -> Option<Decimal> {
  global::engine().lock().pairs.get(symbol).map(|v| v.short.margin)
}

#[pyfunction]
#[pyo3(signature = (symbol))]
pub fn position_short_pnl(symbol: &str) -> Option<Decimal> {
  global::engine().lock().pairs.get(symbol).map(|v| v.short.pnl)
}

#[pyfunction]
#[pyo3(signature = (symbol))]
pub fn pair_leverage(symbol: &str) -> Option<Decimal> {
  global::engine().lock().pairs.get(symbol).map(|v| v.leverage)
}

#[pyfunction]
#[pyo3(signature = (symbol))]
pub fn pair_margin(symbol: &str) -> Option<Decimal> {
  global::engine().lock().pairs.get(symbol).map(|v| v.margin)
}

#[pyfunction]
#[pyo3(signature = (symbol))]
pub fn pair_mark_price(symbol: &str) -> Option<Decimal> {
  global::engine().lock().pairs.get(symbol).map(|v| v.mark_price)
}

#[pyfunction]
#[pyo3(signature = (symbol, id))]
pub fn pair_order(symbol: &str, id: &str) -> Option<Order> {
  global::engine().lock().pairs.get(symbol).map(|v| v.orders.get(id)).flatten().cloned()
}

#[pyfunction]
#[pyo3(signature = (symbol))]
pub fn pair_open_orders(symbol: &str) -> Vec<Order> {
  global::engine()
    .lock()
    .pairs
    .get(symbol)
    .map(|v| {
      v.orders
        .iter()
        .filter_map(|(_, o)| {
          if matches!(
            o.status,
            OrderStatus::Created |
              OrderStatus::Submited |
              OrderStatus::Pending |
              OrderStatus::Partial
          ) {
            Some(o)
          } else {
            None
          }
        })
        .cloned()
        .collect()
    })
    .unwrap_or_default()
}

#[pyfunction]
#[pyo3(signature = (symbol))]
pub fn pair_order_ids(symbol: &str) -> Vec<String> {
  global::engine()
    .lock()
    .pairs
    .get(symbol)
    .map(|v| v.orders.keys().cloned().collect())
    .unwrap_or_default()
}

#[pyfunction]
#[pyo3(signature = ())]
pub fn is_running() -> bool {
  global::engine().lock().running
}

#[pyfunction]
#[pyo3(signature = ())]
pub fn trade_time() -> DateTime<Utc> {
  global::engine().lock().trade_time
}

#[pyfunction]
#[pyo3(signature = ())]
pub fn benchmark() -> String {
  global::engine().lock().benchmark.to_owned()
}

#[pyfunction]
#[pyo3(signature = ())]
pub fn symbols() -> Vec<String> {
  global::engine().lock().pairs.keys().cloned().collect()
}

#[pyfunction]
#[pyo3(signature = (mode, strategy))]
pub fn run(py: Python, mode: Mode, strategy: &str) -> Result<()> {
  py.allow_threads(|| {
    crate::engine::start(mode, strategy)?;
    anyhow::Ok(())
  })?;
  Ok(())
}
