use serde::{Deserialize, Serialize};
use chrono::DateTime;
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug)]
pub struct Netlifytest {
    pub timestamp: DateTime<Utc>,
    pub status_code: u16
}

pub fn run_netlifytest() -> Netlifytest {
    use chrono::prelude::*;

    Netlifytest { timestamp: Utc::now(), status_code: 200 }
}