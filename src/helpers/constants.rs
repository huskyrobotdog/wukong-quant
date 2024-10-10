use strum::Display;

/// 环境变量
#[derive(Debug, Clone, Display)]
pub enum Environment {
  /// 日志级别, 默认`debug`
  #[strum(to_string = "LOG_LEVEL")]
  LogLevel,
  /// 日志颜色, 默认开启
  #[strum(to_string = "LOG_COLOR")]
  LogColor,
  /// 日志显示毫秒时间, 默认关闭
  #[strum(to_string = "LOG_MS")]
  LogMs,
  /// 显示banner, 默认开启
  #[strum(to_string = "SHOW_BANNER")]
  ShowBanner,
}

#[allow(unused)]
impl Environment {
  pub fn value(&self) -> Option<String> {
    std::env::var(self.to_string()).ok()
  }

  pub fn as_bool(&self, default: bool) -> bool {
    self.value().map(|v| v == "true").unwrap_or(default)
  }

  pub fn as_i64(&self, default: i64) -> i64 {
    self
      .value()
      .map(|v| v.parse::<i64>().expect(format!("类型转换失败 : {v}").as_str()))
      .unwrap_or(default)
  }

  pub fn as_usize(&self, default: usize) -> usize {
    self
      .value()
      .map(|v| v.parse::<usize>().expect(format!("类型转换失败 : {v}").as_str()))
      .unwrap_or(default)
  }

  pub fn set_value(&self, val: &str) {
    std::env::set_var(self.to_string(), val);
  }
}

/// banner
pub const BANNER: &str = "
██╗    ██╗██╗   ██╗██╗  ██╗ ██████╗ ███╗   ██╗ ██████╗         
██║    ██║██║   ██║██║ ██╔╝██╔═══██╗████╗  ██║██╔════╝         
██║ █╗ ██║██║   ██║█████╔╝ ██║   ██║██╔██╗ ██║██║  ███╗        
██║███╗██║██║   ██║██╔═██╗ ██║   ██║██║╚██╗██║██║   ██║        
╚███╔███╔╝╚██████╔╝██║  ██╗╚██████╔╝██║ ╚████║╚██████╔╝        
 ╚══╝╚══╝  ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝ ╚═════╝         
                                                               
██████╗ ██╗   ██╗    ██╗  ██╗██╗   ██╗███████╗██╗  ██╗██╗   ██╗
██╔══██╗╚██╗ ██╔╝    ██║  ██║██║   ██║██╔════╝██║ ██╔╝╚██╗ ██╔╝
██████╔╝ ╚████╔╝     ███████║██║   ██║███████╗█████╔╝  ╚████╔╝ 
██╔══██╗  ╚██╔╝      ██╔══██║██║   ██║╚════██║██╔═██╗   ╚██╔╝  
██████╔╝   ██║       ██║  ██║╚██████╔╝███████║██║  ██╗   ██║██╗
╚═════╝    ╚═╝       ╚═╝  ╚═╝ ╚═════╝ ╚══════╝╚═╝  ╚═╝   ╚═╝╚═╝
                                                               
";
