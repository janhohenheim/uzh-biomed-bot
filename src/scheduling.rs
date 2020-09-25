use crate::persistence::*;
use crate::telegram::*;

use chrono::Local;
use clokwerk::Interval::*;
use clokwerk::{ScheduleHandle, Scheduler};
use std::time::Duration;
use tokio::runtime::Runtime;

pub fn schedule_maths() -> ScheduleHandle {
    let mut scheduler = Scheduler::with_tz(Local::now().timezone());
    scheduler
        .every(Tuesday)
        .at("10:00")
        .and_every(Wednesday)
        .at("10:00")
        .run(move || {
            println!("Let's send maths!");
            let current_date = format!("{}", Local::now().format("%Y-%m-%d"));
            let module = read_module("MAT 182")
                .expect("Failed to read modules file")
                .expect("Failed to find module MAT 182 in modules file");
            let link = module
                .live_streams
                .into_iter()
                .find(|live_stream| live_stream.date == current_date)
                .map(|live_stream| live_stream.link);

            let view_model = LiveStreamViewModel {
                identifier: module.identifier,
                name: module.name,
                link,
            };
            Runtime::new()
                .expect("Failed to create Tokio runtime")
                .block_on(broadcast_live_stream(view_model))
                .expect("Failed to broadcast message");
        });
    println!("Scheduled stuff");
    scheduler.watch_thread(Duration::from_millis(100))
}
