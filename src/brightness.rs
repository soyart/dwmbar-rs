use crate::sysfs;
use std::path::Path;

const PATTERN: &str = "/sys/class/backlight/*";

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

fn get_brightness(path: &Path) -> Option<(usize, usize)> {
    let value = {
        let s: &str = &sysfs::read_file(&path.join("brightness"))?;
        s.trim().parse().ok()
    }?;
    let max = {
        let s: &str = &sysfs::read_file(&path.join("max_brightness"))?;
        s.trim().parse().ok()
    }?;

    Some((max, value))
}

pub(crate) fn get() -> String {
    let backlight_paths = sysfs::find_matches(PATTERN);
    if backlight_paths.is_empty() {
        return String::from("null");
    }
    let brightnesses: Vec<(usize, usize)> = backlight_paths
        .iter()
        .map(|pb| pb.as_path())
        .filter_map(get_brightness)
        .collect();

    if brightnesses.is_empty() {
        return String::from("null");
    }

    Brightness(brightnesses).to_string()
}
