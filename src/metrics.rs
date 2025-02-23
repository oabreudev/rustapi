use parking_lot::RwLock;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct Metrics {
    requests: RwLock<u64>,
    errors: RwLock<u64>,
    response_times: RwLock<Vec<Duration>>,
}

impl Metrics {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            requests: RwLock::new(0),
            errors: RwLock::new(0),
            response_times: RwLock::new(Vec::with_capacity(1000)),
        })
    }

    pub fn record_request(&self) {
        *self.requests.write() += 1;
    }

    pub fn record_error(&self) {
        *self.errors.write() += 1;
    }

    pub fn record_response_time(&self, duration: Duration) {
        self.response_times.write().push(duration);
    }

    pub fn get_stats(&self) -> (u64, u64, Duration) {
        let times = self.response_times.read();
        let avg_time = if !times.is_empty() {
            times.iter().sum::<Duration>() / times.len() as u32
        } else {
            Duration::from_secs(0)
        };

        (*self.requests.read(), *self.errors.read(), avg_time)
    }
}

pub struct RequestTimer {
    start: Instant,
    metrics: Arc<Metrics>,
}

impl RequestTimer {
    pub fn new(metrics: Arc<Metrics>) -> Self {
        metrics.record_request();
        Self {
            start: Instant::now(),
            metrics,
        }
    }
}

impl Drop for RequestTimer {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        self.metrics.record_response_time(duration);
    }
}