use time::{
    OffsetDateTime,
    format_description,
};

pub const CLOCK_DEFAULT: &str = "[weekday], [month repr:short] [day padding:zero] > [hour repr:24 padding:zero]:[minute padding:zero]";

pub struct Clock {
    time: OffsetDateTime,
    format: String,
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let clock_format = if self.format.is_empty() {
            CLOCK_DEFAULT
        } else {
            &self.format
        };
        let format = format_description::parse(clock_format).unwrap();
        write!(f, "{}", self.time.format(&format).unwrap())
    }
}

pub fn new_clock(format: &str) -> Clock {
    Clock {
        time: OffsetDateTime::now_local().unwrap(),
        format: format.to_string(),
    }
}

pub(crate) fn get(format: String) -> impl Fn() -> String {
    move || new_clock(&format).to_string()
}
