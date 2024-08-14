use std::time::SystemTime;

#[derive(Debug)]
pub struct Logs {
    pub system_time_nanoseconds: u128,
}

impl Logs {
    pub fn new() -> Result<Self, &'static str> {
        let duration_since_epoch = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(d) => d,
            Err(_) => return Err("a"),
        };
        Ok(Self {
            system_time_nanoseconds: duration_since_epoch.as_nanos(),
        })
    }
}
