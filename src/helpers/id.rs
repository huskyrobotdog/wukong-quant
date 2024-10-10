use uuid::Uuid;

/// 唯一UUID
pub fn gen() -> String {
  Uuid::new_v4().to_string().replace("-", "")
}
