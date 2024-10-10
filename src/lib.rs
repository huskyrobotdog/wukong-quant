mod api;
mod engine;
mod global;
mod helpers;
mod models;
mod types;

use pyo3::prelude::*;

#[pymodule]
fn wukong(m: &Bound<'_, PyModule>) -> PyResult<()> {
  m.add("BANNER", helpers::constants::BANNER)?;
  m.add_class::<types::Mode>()?;
  m.add_class::<types::Type>()?;
  m.add_class::<types::Side>()?;
  m.add_class::<types::TimeFrame>()?;
  m.add_class::<types::OrderStatus>()?;
  m.add_function(wrap_pyfunction!(api::debug, m)?)?;
  m.add_function(wrap_pyfunction!(api::info, m)?)?;
  m.add_function(wrap_pyfunction!(api::warn, m)?)?;
  m.add_function(wrap_pyfunction!(api::error, m)?)?;
  m.add_function(wrap_pyfunction!(api::print, m)?)?;
  m.add_function(wrap_pyfunction!(api::str_to_date, m)?)?;
  m.add_function(wrap_pyfunction!(api::ms_to_date, m)?)?;
  m.add_function(wrap_pyfunction!(api::now_ms, m)?)?;
  m.add_function(wrap_pyfunction!(api::rand_id, m)?)?;
  m.add_function(wrap_pyfunction!(api::account_cash, m)?)?;
  m.add_function(wrap_pyfunction!(api::account_available_cash, m)?)?;
  m.add_function(wrap_pyfunction!(api::account_margin, m)?)?;
  m.add_function(wrap_pyfunction!(api::account_pnl, m)?)?;
  m.add_function(wrap_pyfunction!(api::position_long_size, m)?)?;
  m.add_function(wrap_pyfunction!(api::position_long_available_size, m)?)?;
  m.add_function(wrap_pyfunction!(api::position_long_price, m)?)?;
  m.add_function(wrap_pyfunction!(api::position_long_margin, m)?)?;
  m.add_function(wrap_pyfunction!(api::position_long_pnl, m)?)?;
  m.add_function(wrap_pyfunction!(api::position_short_size, m)?)?;
  m.add_function(wrap_pyfunction!(api::position_short_available_size, m)?)?;
  m.add_function(wrap_pyfunction!(api::position_short_price, m)?)?;
  m.add_function(wrap_pyfunction!(api::position_short_margin, m)?)?;
  m.add_function(wrap_pyfunction!(api::position_short_pnl, m)?)?;
  m.add_function(wrap_pyfunction!(api::pair_leverage, m)?)?;
  m.add_function(wrap_pyfunction!(api::pair_margin, m)?)?;
  m.add_function(wrap_pyfunction!(api::pair_mark_price, m)?)?;
  m.add_function(wrap_pyfunction!(api::pair_order, m)?)?;
  m.add_function(wrap_pyfunction!(api::pair_open_orders, m)?)?;
  m.add_function(wrap_pyfunction!(api::pair_order_ids, m)?)?;
  m.add_function(wrap_pyfunction!(api::is_running, m)?)?;
  m.add_function(wrap_pyfunction!(api::trade_time, m)?)?;
  m.add_function(wrap_pyfunction!(api::benchmark, m)?)?;
  m.add_function(wrap_pyfunction!(api::symbols, m)?)?;
  m.add_function(wrap_pyfunction!(api::run, m)?)?;
  Ok(())
}
