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
  m.add_function(wrap_pyfunction!(api::str_to_date, m)?)?;
  m.add_function(wrap_pyfunction!(api::ms_to_date, m)?)?;
  m.add_function(wrap_pyfunction!(api::now_ms, m)?)?;
  m.add_function(wrap_pyfunction!(api::rand_id, m)?)?;
  Ok(())
}
