use crate::sysfs;
use std::path::PathBuf;

const PATTERN: &str = "/sys/class/hwmon/hwmon*/fan*_input";
const DEFAULT_LIMIT: usize = 2;

// Fans represents RPM values for multiple fans
pub(crate) struct Fans(pub(crate) Vec<u32>);

impl std::fmt::Display for Fans {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return write!(f, "rpm: no data");
        }
        write!(f, "rpm:")?;
        for rpm in &self.0 {
            write!(f, " {}", rpm)?;
        }
        Ok(())
    }
}

fn get_fans(fan_paths: &[PathBuf], limit: usize) -> Fans {
    let effective_limit = match limit {
        0 => fan_paths.len(),
        _ => limit.min(fan_paths.len()),
    };

    let mut rpms = Vec::with_capacity(effective_limit);
    for fan_file in fan_paths.iter().take(effective_limit) {
        if let Some(state) = sysfs::read_file(fan_file) {
            if state.is_empty() {
                continue;
            }
            if let Some(rpm) = {
                let s: &str = &state;
                s.trim().parse().ok()
            } {
                rpms.push(rpm);
            }
        }
    }

    Fans(rpms)
}

pub(crate) fn get() -> String {
    let fan_paths = sysfs::find_matches(PATTERN);
    if fan_paths.is_empty() {
        return String::from("rpm: no data");
    }
    get_fans(&fan_paths, DEFAULT_LIMIT).to_string()
}
