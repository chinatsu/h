use std::env;

pub struct Environment {
    pub log_conf: String,
    pub brokers: String,
    pub topic: String,
    pub messages: usize
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            log_conf: env::var("LOG_CONF").unwrap_or("info".to_string()),
            brokers: env::var("BROKERS").unwrap_or("localhost:29092".to_string()),
            topic: env::var("TOPIC").unwrap_or("h".to_string()),
            messages: env::var("MESSAGES").unwrap_or("10000".to_string()).parse::<usize>().unwrap_or(10_000)
        }
    }
}
