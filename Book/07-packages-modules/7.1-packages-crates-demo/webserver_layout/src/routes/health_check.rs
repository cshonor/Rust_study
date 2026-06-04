use crate::domain::Subscriber;

pub fn handler_name() -> &'static str {
    "health_check"
}

pub fn log_ready() {
    let _sub = Subscriber::new("demo@example.com");
    println!("[routes] {} ready", handler_name());
}
