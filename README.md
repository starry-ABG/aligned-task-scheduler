# Aligned Task Scheduler

`AlignedTaskScheduler` is a scheduler built on the [Tokio](https://tokio.rs/) asynchronous runtime that runs tasks at precise minute intervals, with an optional offset in seconds. No matter when your application starts, it will first align to the next interval boundary before running tasks, ensuring tasks always trigger at exact times.

## Features

- **Precise Alignment**: Run tasks at exact minute boundaries, suitable for scheduled reporting, syncing, or periodic checks.
- **Configurable Offset**: Add a few seconds of delay after the exact minute mark, allowing for minor adjustments to the schedule.
- **Asynchronous Support**: Built on Tokio’s async runtime, making it straightforward to perform I/O, network calls, or database operations in your scheduled tasks.

## Installation

Add the dependency in your `Cargo.toml`:

```toml
[dependencies]
aligned-task-scheduler = "0.1.0"
tokio = { version = "1", features = ["macros", "rt-multi-thread", "time"] }
```

## Quick Start Example

In the example below, we create a scheduler that triggers a task every 5 minutes plus a 10-second offset. The task prints the aligned timestamp and the actual execution time:

```rust
use aligned_task_scheduler::AlignedTaskScheduler;
use std::time::SystemTime;

#[tokio::main]
async fn main() {
    // Create a scheduler that runs tasks every 5 minutes + 10 seconds offset
    let scheduler = AlignedTaskScheduler::new(5, 10);

    // Provide an async closure that executes after each alignment
    scheduler.run(|timestamp| async move {
        println!("Task executed at aligned timestamp: {}", timestamp);
        println!("Actual current time: {:?}", SystemTime::now());
        // Insert your business logic here:
        // e.g., fetch data, process results, write to a database, etc.
    }).await;
}
```