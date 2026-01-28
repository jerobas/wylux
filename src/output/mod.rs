pub mod json;
pub mod human;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputMode {
    Human,
    Json,
}

pub trait OutputFormatter {
    fn format(&self, mode: OutputMode) -> String;
}
