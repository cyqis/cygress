use prometheus::register_int_counter;

pub struct Metric {
    pub req_metric: prometheus::IntCounter,
}

impl Metric {
    pub fn new() -> Self {
        Self {
            req_metric: register_int_counter!("req_counter", "Number of requests").unwrap(),
        }
    }
}
