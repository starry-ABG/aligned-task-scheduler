# 对齐任务调度器 (Aligned Task Scheduler)

`AlignedTaskScheduler` 是一个基于 [Tokio](https://tokio.rs/) 异步运行时构建的调度器，它能够在精确的整分钟间隔运行任务，并可选地在整分钟基础上增加数秒偏移。无论你的程序何时启动，它都会先对齐到下一个整分钟边界后才执行任务，确保任务总是在精确的时间点触发。

## 功能特性

- **精确对齐**：在整分钟边界运行任务，适用于定期报告、同步或周期性检查的场景。
- **可配置偏移**：在整分钟时间点后增加几秒延迟，以根据需要对执行时间进行微调。
- **异步支持**：基于 Tokio 异步运行时，使在任务中进行 I/O、网络调用或数据库操作变得轻松易行。

## 安装

在你的 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
aligned-task-scheduler = "0.1.0"
tokio = { version = "1", features = ["macros", "rt-multi-thread", "time"] }
```

## 快速开始示例
在下面的示例中，我们创建了一个调度器，每隔 5 分钟并在此基础上延迟 10 秒执行一次任务。任务会打印对齐后的时间戳以及实际的执行时间：

```rust
use aligned_task_scheduler::AlignedTaskScheduler;
use std::time::SystemTime;

#[tokio::main]
async fn main() {
    // 创建一个每隔5分钟 + 10秒偏移执行任务的调度器
    let scheduler = AlignedTaskScheduler::new(5, 10);

    // 提供一个异步闭包，在每次对齐后执行
    scheduler.run(|timestamp| async move {
        println!("任务在对齐的时间戳执行: {}", timestamp);
        println!("实际当前时间: {:?}", SystemTime::now());
        // 在此加入你的业务逻辑：
        // 例如，获取数据、处理结果、写入数据库等
    }).await;
}
```