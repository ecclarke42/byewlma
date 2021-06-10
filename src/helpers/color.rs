#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
    Light,
    Dark,

    Primary,
    PrimaryLight,
    PrimaryDark,

    Link,
    LinkLight,
    LinkDark,

    Info,
    InfoLight,
    InfoDark,

    Success,
    SuccessLight,
    SuccessDark,

    Warning,
    WarningLight,
    WarningDark,

    Danger,
    DangerLight,
    DangerDark,
}

// TODO: trait
impl Color {
    pub fn text_class(&self) -> &'static str {
        use Color::*;
        match self {
            White => "has-text-white",
            Black => "has-text-black",
            Light => "has-text-light",
            Dark => "has-text-dark",

            Primary => "has-text-primary",
            PrimaryLight => "has-text-primary-light",
            PrimaryDark => "has-text-primary-dark",

            Link => "has-text-link",
            LinkLight => "has-text-link-light",
            LinkDark => "has-text-link-dark",

            Info => "has-text-info",
            InfoLight => "has-text-info-light",
            InfoDark => "has-text-info-dark",

            Success => "has-text-success",
            SuccessLight => "has-text-success-light",
            SuccessDark => "has-text-success-dark",

            Warning => "has-text-warning",
            WarningLight => "has-text-warning-light",
            WarningDark => "has-text-warning-dark",

            Danger => "has-text-danger",
            DangerLight => "has-text-danger-light",
            DangerDark => "has-text-danger-dark",
        }
    }

    pub fn background_class(&self) -> &'static str {
        use Color::*;
        match self {
            White => "has-background-white",
            Black => "has-background-black",
            Light => "has-background-light",
            Dark => "has-background-dark",

            Primary => "has-background-primary",
            PrimaryLight => "has-background-primary-light",
            PrimaryDark => "has-background-primary-dark",

            Link => "has-background-link",
            LinkLight => "has-background-link-light",
            LinkDark => "has-background-link-dark",

            Info => "has-background-info",
            InfoLight => "has-background-info-light",
            InfoDark => "has-background-info-dark",

            Success => "has-background-success",
            SuccessLight => "has-background-success-light",
            SuccessDark => "has-background-success-dark",

            Warning => "has-background-warning",
            WarningLight => "has-background-warning-light",
            WarningDark => "has-background-warning-dark",

            Danger => "has-background-danger",
            DangerLight => "has-background-danger-light",
            DangerDark => "has-background-danger-dark",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TextShade {
    BlackBis,
    BlackTer,
    GreyDarker,
    GreyDark,
    Grey,
    GreyLight,
    GreyLigher,
    WhiteTer,
    WhiteBis,
}

impl TextShade {
    pub fn class(&self) -> &'static str {
        match self {
            BlackBis => "has-text-black-bis",
            BlackTer => "has-text-black-ter",
            GreyDarker => "has-text-grey-darker",
            GreyDark => "has-text-grey-dark",
            Grey => "has-text-grey",
            GreyLight => "has-text-grey-light",
            GreyLighter => "has-text-grey-lighter",
            WhiteTer => "has-text-white-ter",
            WhiteBis => "has-text-white-bis",
        }
    }
}

// use crate::Attribute;

// /// Enumeration of standard Bulma color variables
// /// https://bulma.io/documentation/customize/variables/#variables
// #[derive(Clone, Copy, PartialEq, Eq)]
// pub enum Color {
//     Semantic(SemanticColor, Option<Adjustment>),

//     Text(TextColor),
//     Link(LinkColor),

//     White,
//     Black,
//     Light,
//     Dark,

//     Ghost,
// }

// impl Color {
//     pub fn primary() -> Color {
//         Color::Semantic(SemanticColor::Primary, None)
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum SemanticColor {
//     Primary,
//     Info,
//     Success,
//     Warning,
//     Danger,
//     Link,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum TextColor {
//     Defaut,
//     Light,
//     Strong,
//     Invert,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum LinkColor {
//     Defaut,

//     Invert,
//     Light,
//     Dark,
//     Visited,

//     // TODO: Border variants?
//     Hover,
//     Focus,
//     Active,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum Adjustment {
//     Light,
//     Dark,
//     Invert,
// }

// impl Attribute for Color {
//     fn as_classes(&self) -> yew::html::Classes {
//         use Color::*;
//         let cls = match *self {

//             Semantic(SemanticColor, Option<Adjustment>),

//             Text(TextColor),
//             Link(LinkColor),

//             White,
//             Black,
//             Light,
//             Dark,

//             Ghost,

//             White => "has-text-white",
//             Black => "has-text-black",
//             Light => "has-text-light",
//             Dark => "has-text-dark",
//             Primary => "has-text-primary",
//             Link => "has-text-link",
//             Info => "has-text-info",
//             Success => "has-text-success",
//             Warning => "has-text-warning",
//             Danger => "has-text-danger",
//         };
//         yew::classes!(cls)
//     }
// }
