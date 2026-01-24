use chrono::Local;

pub(crate) fn get() -> String {
    Local::now().format("%A, %b %d > %H:%M").to_string()
}
