mod color;
mod size;
mod spacing;
mod typography;

pub use color::*;
pub use size::*;
pub use spacing::*;
pub use typography::{FontFamily, Text, TextAlignment, TextSize, TextTransform, TextWeight};

pub enum Attribute {
    // Color(Color),
    BackgroundColor(Color),
    Margin(Margin),
    Padding(Padding),
}

impl crate::BulmaClass for Attribute {
    fn class(&self) -> &'static str {
        match self {
            // Attribute::Color(color) => color
            Attribute::BackgroundColor(color) => color.background_class(),
            Attribute::Margin(margin) => margin.class(),
            Attribute::Padding(padding) => padding.class(),
        }
    }
}
