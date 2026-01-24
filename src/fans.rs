use crate::sysfs;
use std::fs;
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

fn read_file(path: &PathBuf) -> Option<String> {
    fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}

fn parse_rpm(s: &str) -> Option<u32> {
    s.trim().parse().ok()
}

fn get_fans(fan_paths: &[PathBuf], limit: usize) -> Fans {
    let effective_limit = if limit <= 0 {
        fan_paths.len()
    } else {
        limit.min(fan_paths.len())
    };

    let mut rpms = Vec::with_capacity(effective_limit);
    for fan_file in fan_paths.iter().take(effective_limit) {
        if let Some(state) = read_file(fan_file) {
            if state.is_empty() {
                continue;
            }
            if let Some(rpm) = parse_rpm(&state) {
                rpms.push(rpm);
            }
        }
    }

    Fans(rpms)
}

pub(crate) fn get() -> String {
    let fan_paths = sysfs::find_all_matches(PATTERN);
    if fan_paths.is_empty() {
        return String::from("rpm: no data");
    }

    let fans = get_fans(&fan_paths, DEFAULT_LIMIT);
    fans.to_string()
}
