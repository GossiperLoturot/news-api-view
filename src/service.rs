use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

use ureq::serde_json::Value;

pub struct AppService {
    pub data: Arc<Mutex<Value>>,
    #[allow(dead_code)]
    thread: JoinHandle<()>,
}

impl AppService {
    pub fn new(url: String, interval: Duration) -> AppService {
        let data = Arc::new(Mutex::new(Value::Null));
        let data_clone = Arc::clone(&data);
        let thread = thread::spawn(move || loop {
            if let Some((res, mut data)) = ureq::get(&url).call().ok()
                .and_then(|res| res.into_json::<Value>().ok())
                .and_then(|res| data_clone.lock().ok().map(|data| (res, data)))
            {
                *data = res;
            }
            thread::sleep(interval);
        });
        AppService { data, thread }
    }
}
