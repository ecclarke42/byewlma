use crate::innerlude::*;

mod color;
mod spacing;
mod typography;

pub use color::*;
pub use spacing::*;
pub use typography::{FontFamily, Text, TextAlignment, TextSize, TextTransform, TextWeight};

#[derive(Debug, Clone, Copy, PartialEq, Eq, BulmaClass)]
pub enum Size {
    #[bulma_class = "is-small"]
    Small,

    #[bulma_class = "is-normal"]
    Normal,

    #[bulma_class = "is-medium"]
    Medium,

    #[bulma_class = "is-large"]
    Large,
}
