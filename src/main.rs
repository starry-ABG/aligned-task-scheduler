use std::time::SystemTime;
use tokio::time::sleep;
use std::time::Duration;
use aligned_task_scheduler::AlignedTaskScheduler;

#[tokio::main]
async fn main() {
    let scheduler = AlignedTaskScheduler::new(1, 10); // 每1分钟整点 +10秒时执行任务
    scheduler.run(|ts| async move {
        println!("执行任务的整分时间戳: {}", ts);
        println!("当前真实时间: {:?}", SystemTime::now());
        // 在这里执行你的实际逻辑，比如拉取数据、写入数据库等
    }).await;
}