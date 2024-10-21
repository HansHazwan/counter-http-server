use crate::prelude::*;
use serde::{Serialize, Deserialize};
use tokio::sync::Mutex;

pub struct AppState {
    pub count: Mutex<i32>,
    pub app_name: String,
}

impl AppState {
    pub fn new(app_name: &str) -> AppState {
        AppState {
            count: Mutex::new(0),
            app_name: app_name.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CounterObjectTransfer {
    pub count: i32,
}

impl CounterObjectTransfer {
    pub fn new(count: i32) -> CounterObjectTransfer {
        CounterObjectTransfer {
            count,
        }
    }
}

