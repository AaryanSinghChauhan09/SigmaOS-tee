//! Options model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppendMode {
    Overwrite,
    Append,
}

#[derive(Debug, Clone)]
pub struct TeeOptions {
    pub append_mode: AppendMode,
    pub ignore_interrupts: bool,
}

impl Default for TeeOptions {
    fn default() -> Self {
        Self {
            append_mode: AppendMode::Overwrite,
            ignore_interrupts: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tee_options() {
        let o = TeeOptions::default();
        assert_eq!(o.append_mode, AppendMode::Overwrite);
    }
}
