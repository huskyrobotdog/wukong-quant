use uuid::Uuid;

/// 随机32位ID
pub fn gen() -> String {
  Uuid::new_v4().to_string().replace("-", "")
}
