use crate::server::models::status::StatusBadge;
use std::sync::{Arc, Mutex};
use tokio::time::{Duration, interval};

pub type StatusData = Arc<Mutex<StatusBadge>>;

pub struct StatusService;

impl StatusService {
    pub fn start_status_monitor(status: StatusData) {
        let status_clone = status.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(43_200)); // 12 hours
            loop {
                interval.tick().await;
                let new_status = StatusBadge::check_status().await;
                let mut data = status_clone.lock().unwrap();
                *data = new_status;
            }
        });
    }

    // Add more service methods as needed
}
