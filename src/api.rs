use anyhow::Result;
use chrono::prelude::*;
use pyo3::prelude::*;

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
