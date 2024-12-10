use tokio::time::{sleep, Duration};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct AlignedTaskScheduler {
    interval_minutes: u64,
    offset_seconds: u64,
}

impl AlignedTaskScheduler {
    /// interval_minutes：每隔多少分钟的整点执行
    /// offset_seconds：在整点基础上增加多少秒的偏移执行
    pub fn new(interval_minutes: u64, offset_seconds: u64) -> Self {
        AlignedTaskScheduler { interval_minutes, offset_seconds }
    }

    /// 运行单个异步任务
    /// task：返回()`的异步闭包
    pub async fn run<F, Fut>(&self, mut task: F) -> !
    where
        F: FnMut(u64) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        loop {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            let interval_secs = self.interval_minutes * 60;

            // 下一个整 interval_minutes 分钟的时间戳
            let next_interval = ((now / interval_secs) + 1) * interval_secs;

            let wait_secs_to_interval = next_interval - now;
            // 等待直到整点
            sleep(Duration::from_secs(wait_secs_to_interval)).await;

            // 等待偏移秒数
            if self.offset_seconds > 0 {
                sleep(Duration::from_secs(self.offset_seconds)).await;
            }

            // 执行单个任务
            task(next_interval).await;
        }
    }
}