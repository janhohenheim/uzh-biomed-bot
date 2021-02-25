use crate::telegram::*;

use chrono::Local;
use clokwerk::Interval::*;
use clokwerk::{ScheduleHandle, Scheduler};
use std::time::Duration;
use tokio::runtime::Runtime;

pub fn schedule_maths() -> ScheduleHandle {
    let mut scheduler = Scheduler::with_tz(Local::now().timezone());
    scheduler
        .every(Wednesday)
        .at("10:00")
        .and_every(Friday)
        .at("10:00")
        .run(move || schedule_module(LiveStreamViewModel{
            identifier: "MAT 183".to_owned(),
            name: "Stochastik für die Naturwissenschaften".to_owned(),
            link: Some("https://lms.uzh.ch/auth/RepositoryEntry/16974184862/CourseNode/103233511448483".to_owned()),
            password: None
        }));
    scheduler.every(Tuesday).at("10:00").run(move || {
        schedule_module(LiveStreamViewModel {
            identifier: "BIO 124".to_owned(),
            name: "Einführung in die Ethik und Theorie der Biologie".to_owned(),
            link: Some(
                "https://uzh.zoom.us/j/91884901277?pwd=V2xuRmh0WkdiSWV3VHEvK05hY1R2QT09".to_owned(),
            ),
            password: Some("095870".to_owned()),
        })
    });
    scheduler
        .every(Tuesday)
        .at("07:45")
        .and_every(Wednesday)
        .at("07:45")
        .run(move || {
            schedule_module(LiveStreamViewModel {
                identifier: "CHE 127".to_owned(),
                name: "Organische Chemie für die Life Sciences".to_owned(),
                link: Some(
                    "https://uzh.zoom.us/s/92605143274?pwd=cjc1OUlSZFRSZ3FqdDU0aE54K0VWQT09"
                        .to_owned(),
                ),
                password: Some("864624".to_owned()),
            })
        });
    scheduler.every(Tuesday).at("10:00").run(move || {
        schedule_module(LiveStreamViewModel {
            identifier: "BIO 122".to_owned(),
            name: "Verhaltensbiologie ".to_owned(),
            link: Some(
                "https://uzh.zoom.us/j/96544137154?pwd=S0U0UXo1UVVCaEZqNkZvUFQ0NmdWQT09".to_owned(),
            ),
            password: Some("133158".to_owned()),
        })
    });
    scheduler.watch_thread(Duration::from_millis(100))
}

fn schedule_module(view_model: LiveStreamViewModel) {
    let broadcast_result = Runtime::new()
        .expect("Failed to create Tokio runtime")
        .block_on(broadcast_live_stream(view_model));
    if let Err(error) = broadcast_result {
        println!("An error happened while broadcasting: {}", error)
    }
}
