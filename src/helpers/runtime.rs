use anyhow::Result;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::runtime::{Builder, Runtime};

pub fn init_rayon() -> Result<()> {
  let nums = num_cpus::get();
  rayon::ThreadPoolBuilder::new()
    .thread_name(|n| format!("WuKong-Computation-{}", n))
    .num_threads(if nums < 2 { 2 } else { nums })
    .build_global()?;
  Ok(())
}

pub fn tokio() -> &'static Runtime {
  &TOKIO
}

static TOKIO: Lazy<Runtime> = Lazy::new(|| {
  let nums = num_cpus::get();
  let rt = Builder::new_multi_thread()
    .thread_name_fn(|| {
      static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
      let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
      format!("WuKong-Runtime-{}", id)
    })
    .worker_threads(if nums < 2 { 2 } else { nums })
    .max_blocking_threads(if nums < 2 { 2 } else { nums })
    .enable_all()
    .build()
    .expect("初始化运行线程池失败");
  rt
});
