use crate::Attribute;

/// Enumeration of standard Bulma color variables
/// https://bulma.io/documentation/customize/variables/#variables
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Semantic(SemanticColor, Option<Adjustment>),

    Text(TextColor),
    Link(LinkColor),

    White,
    Black,
    Light,
    Dark,

    Ghost,
}

impl Color {
    pub fn primary() -> Color {
        Color::Semantic(SemanticColor::Primary, None)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticColor {
    Primary,
    Info,
    Success,
    Warning,
    Danger,
    Link,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextColor {
    Defaut,
    Light,
    Strong,
    Invert,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkColor {
    Defaut,

    Invert,
    Light,
    Dark,
    Visited,

    // TODO: Border variants?
    Hover,
    Focus,
    Active,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Adjustment {
    Light,
    Dark,
    Invert,
}

impl Attribute for Color {
    fn as_classes(&self) -> yew::html::Classes {
        use Color::*;
        let cls = match *self {
            
            Semantic(SemanticColor, Option<Adjustment>),

            Text(TextColor),
            Link(LinkColor),

            White,
            Black,
            Light,
            Dark,

            Ghost,

            White => "has-text-white",
            Black => "has-text-black",
            Light => "has-text-light",
            Dark => "has-text-dark",
            Primary => "has-text-primary",
            Link => "has-text-link",
            Info => "has-text-info",
            Success => "has-text-success",
            Warning => "has-text-warning",
            Danger => "has-text-danger",
        };
        yew::classes!(cls)
    }
}
