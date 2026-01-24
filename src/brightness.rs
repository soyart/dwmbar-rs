use crate::sysfs;
use std::fs;
use std::path::PathBuf;

// Brightness represents brightness on multiple displays
pub(crate) struct Brightness(pub(crate) Vec<(usize, usize)>);

impl std::fmt::Display for Brightness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, &(max, val)) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, " ")?;
            }
            write!(f, "[{}]:{:.0}%", i, val as f32 / max as f32 * 100.0)?
        }
        Ok(())
    }
}

fn read_file(path: &PathBuf) -> Option<String> {
    fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}

fn parse_uint(s: &str) -> Option<usize> {
    s.trim().parse().ok()
}

fn get_brightness(path: &PathBuf) -> Option<(usize, usize)> {
    let brightness_file = path.join("brightness");
    let max_brightness_file = path.join("max_brightness");

    let value_str = read_file(&brightness_file)?;
    let max_str = read_file(&max_brightness_file)?;

    let value = parse_uint(&value_str)?;
    let max = parse_uint(&max_str)?;

    Some((max, value))
}

pub(crate) fn get() -> String {
    const PATTERN: &str = "/sys/class/backlight/*";
    let backlight_paths = sysfs::find_all_matches(PATTERN);
    if backlight_paths.is_empty() {
        return String::from("null");
    }
    let brightnesses: Vec<(usize, usize)> =
        backlight_paths.iter().filter_map(get_brightness).collect();

    if brightnesses.is_empty() {
        return String::from("null");
    }
    Brightness(brightnesses).to_string()
}
