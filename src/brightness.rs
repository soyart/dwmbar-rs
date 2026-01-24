// Brightness represents brightness on multiple displays
pub struct Brightness(pub Vec<(usize, usize)>);

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
