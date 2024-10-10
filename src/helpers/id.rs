use pyo3::pyfunction;
use uuid::Uuid;

/// 唯一UUID
#[pyfunction]
#[pyo3(name = "rand_uid", signature = ())]
pub fn gen() -> String {
  Uuid::new_v4().to_string().replace("-", "")
}
