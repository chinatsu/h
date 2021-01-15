use std::env;

pub struct Environment {
    pub log_conf: String,
    pub brokers: String,
    pub topic: String
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            log_conf: env::var("LOG_CONF").unwrap_or("rdkafka=warn".to_string()),
            brokers: env::var("BROKERS").unwrap_or("localhost:29092".to_string()),
            topic: env::var("TOPIC").unwrap_or("h".to_string())
        }
    }
}
