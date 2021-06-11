#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
    Light,
    Dark,
    Semantic(ShadedSemanticColor),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SemanticColor {
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,
}

impl SemanticColor {
    pub fn to_light(self) -> SemanticOrLightColor {
        SemanticOrLightColor::Light(self)
    }

    pub fn is_class(&self) -> &'static str {
        use SemanticColor::*;
        match self {
            Primary => "is-primary",
            Link => "is-link",
            Info => "is-info",
            Success => "is-success",
            Warning => "is-warning",
            Danger => "is-danger",
        }
    }
}

impl Color {
    pub const PRIMARY: Self = Self::Semantic(ShadedSemanticColor::PRIMARY);
    pub const LINK: Self = Self::Semantic(ShadedSemanticColor::LINK);
    pub const INFO: Self = Self::Semantic(ShadedSemanticColor::INFO);
    pub const SUCCESS: Self = Self::Semantic(ShadedSemanticColor::SUCCESS);
    pub const WARNING: Self = Self::Semantic(ShadedSemanticColor::WARNING);
    pub const DANGER: Self = Self::Semantic(ShadedSemanticColor::DANGER);

    pub const PRIMARY_LIGHT: Self = Self::Semantic(ShadedSemanticColor::PRIMARY_LIGHT);
    pub const LINK_LIGHT: Self = Self::Semantic(ShadedSemanticColor::LINK_LIGHT);
    pub const INFO_LIGHT: Self = Self::Semantic(ShadedSemanticColor::INFO_LIGHT);
    pub const SUCCESS_LIGHT: Self = Self::Semantic(ShadedSemanticColor::SUCCESS_LIGHT);
    pub const WARNING_LIGHT: Self = Self::Semantic(ShadedSemanticColor::WARNING_LIGHT);
    pub const DANGER_LIGHT: Self = Self::Semantic(ShadedSemanticColor::DANGER_LIGHT);

    pub const PRIMARY_DARK: Self = Self::Semantic(ShadedSemanticColor::PRIMARY_DARK);
    pub const LINK_DARK: Self = Self::Semantic(ShadedSemanticColor::LINK_DARK);
    pub const INFO_DARK: Self = Self::Semantic(ShadedSemanticColor::INFO_DARK);
    pub const SUCCESS_DARK: Self = Self::Semantic(ShadedSemanticColor::SUCCESS_DARK);
    pub const WARNING_DARK: Self = Self::Semantic(ShadedSemanticColor::WARNING_DARK);
    pub const DANGER_DARK: Self = Self::Semantic(ShadedSemanticColor::DANGER_DARK);

    pub fn text_class(&self) -> &'static str {
        use Color::*;
        match self {
            White => "has-text-white",
            Black => "has-text-black",
            Light => "has-text-light",
            Dark => "has-text-dark",

            Semantic(ShadedSemanticColor::Normal(SemanticColor::Primary)) => "has-text-primary",
            Semantic(ShadedSemanticColor::Light(SemanticColor::Primary)) => {
                "has-text-primary-light"
            }
            Semantic(ShadedSemanticColor::Dark(SemanticColor::Primary)) => "has-text-primary-dark",

            Semantic(ShadedSemanticColor::Normal(SemanticColor::Link)) => "has-text-link",
            Semantic(ShadedSemanticColor::Light(SemanticColor::Link)) => "has-text-link-light",
            Semantic(ShadedSemanticColor::Dark(SemanticColor::Link)) => "has-text-link-dark",

            Semantic(ShadedSemanticColor::Normal(SemanticColor::Info)) => "has-text-info",
            Semantic(ShadedSemanticColor::Light(SemanticColor::Info)) => "has-text-info-light",
            Semantic(ShadedSemanticColor::Dark(SemanticColor::Info)) => "has-text-info-dark",

            Semantic(ShadedSemanticColor::Normal(SemanticColor::Success)) => "has-text-success",
            Semantic(ShadedSemanticColor::Light(SemanticColor::Success)) => {
                "has-text-success-light"
            }
            Semantic(ShadedSemanticColor::Dark(SemanticColor::Success)) => "has-text-success-dark",

            Semantic(ShadedSemanticColor::Normal(SemanticColor::Warning)) => "has-text-warning",
            Semantic(ShadedSemanticColor::Light(SemanticColor::Warning)) => {
                "has-text-warning-light"
            }
            Semantic(ShadedSemanticColor::Dark(SemanticColor::Warning)) => "has-text-warning-dark",

            Semantic(ShadedSemanticColor::Normal(SemanticColor::Danger)) => "has-text-danger",
            Semantic(ShadedSemanticColor::Light(SemanticColor::Danger)) => "has-text-danger-light",
            Semantic(ShadedSemanticColor::Dark(SemanticColor::Danger)) => "has-text-danger-dark",
        }
    }

    pub fn background_class(&self) -> &'static str {
        use Color::*;
        match self {
            White => "has-background-white",
            Black => "has-background-black",
            Light => "has-background-light",
            Dark => "has-background-dark",

            Semantic(ShadedSemanticColor::Normal(SemanticColor::Primary)) => {
                "has-background-primary"
            }
            Semantic(ShadedSemanticColor::Light(SemanticColor::Primary)) => {
                "has-background-primary-light"
            }
            Semantic(ShadedSemanticColor::Dark(SemanticColor::Primary)) => {
                "has-background-primary-dark"
            }

            Semantic(ShadedSemanticColor::Normal(SemanticColor::Link)) => "has-background-link",
            Semantic(ShadedSemanticColor::Light(SemanticColor::Link)) => {
                "has-background-link-light"
            }
            Semantic(ShadedSemanticColor::Dark(SemanticColor::Link)) => "has-background-link-dark",

            Semantic(ShadedSemanticColor::Normal(SemanticColor::Info)) => "has-background-info",
            Semantic(ShadedSemanticColor::Light(SemanticColor::Info)) => {
                "has-background-info-light"
            }
            Semantic(ShadedSemanticColor::Dark(SemanticColor::Info)) => "has-background-info-dark",

            Semantic(ShadedSemanticColor::Normal(SemanticColor::Success)) => {
                "has-background-success"
            }
            Semantic(ShadedSemanticColor::Light(SemanticColor::Success)) => {
                "has-background-success-light"
            }
            Semantic(ShadedSemanticColor::Dark(SemanticColor::Success)) => {
                "has-background-success-dark"
            }

            Semantic(ShadedSemanticColor::Normal(SemanticColor::Warning)) => {
                "has-background-warning"
            }
            Semantic(ShadedSemanticColor::Light(SemanticColor::Warning)) => {
                "has-background-warning-light"
            }
            Semantic(ShadedSemanticColor::Dark(SemanticColor::Warning)) => {
                "has-background-warning-dark"
            }

            Semantic(ShadedSemanticColor::Normal(SemanticColor::Danger)) => "has-background-danger",
            Semantic(ShadedSemanticColor::Light(SemanticColor::Danger)) => {
                "has-background-danger-light"
            }
            Semantic(ShadedSemanticColor::Dark(SemanticColor::Danger)) => {
                "has-background-danger-dark"
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ShadedSemanticColor {
    Light(SemanticColor),
    Normal(SemanticColor),
    Dark(SemanticColor),
}

impl ShadedSemanticColor {
    pub const PRIMARY: Self = Self::Normal(SemanticColor::Primary);
    pub const LINK: Self = Self::Normal(SemanticColor::Link);
    pub const INFO: Self = Self::Normal(SemanticColor::Info);
    pub const SUCCESS: Self = Self::Normal(SemanticColor::Success);
    pub const WARNING: Self = Self::Normal(SemanticColor::Warning);
    pub const DANGER: Self = Self::Normal(SemanticColor::Danger);

    pub const PRIMARY_LIGHT: Self = Self::Light(SemanticColor::Primary);
    pub const LINK_LIGHT: Self = Self::Light(SemanticColor::Link);
    pub const INFO_LIGHT: Self = Self::Light(SemanticColor::Info);
    pub const SUCCESS_LIGHT: Self = Self::Light(SemanticColor::Success);
    pub const WARNING_LIGHT: Self = Self::Light(SemanticColor::Warning);
    pub const DANGER_LIGHT: Self = Self::Light(SemanticColor::Danger);

    pub const PRIMARY_DARK: Self = Self::Dark(SemanticColor::Primary);
    pub const LINK_DARK: Self = Self::Dark(SemanticColor::Link);
    pub const INFO_DARK: Self = Self::Dark(SemanticColor::Info);
    pub const SUCCESS_DARK: Self = Self::Dark(SemanticColor::Success);
    pub const WARNING_DARK: Self = Self::Dark(SemanticColor::Warning);
    pub const DANGER_DARK: Self = Self::Dark(SemanticColor::Danger);
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SemanticOrLightColor {
    Normal(SemanticColor),
    Light(SemanticColor),
}

impl SemanticOrLightColor {
    pub const PRIMARY: Self = Self::Normal(SemanticColor::Primary);
    pub const LINK: Self = Self::Normal(SemanticColor::Link);
    pub const INFO: Self = Self::Normal(SemanticColor::Info);
    pub const SUCCESS: Self = Self::Normal(SemanticColor::Success);
    pub const WARNING: Self = Self::Normal(SemanticColor::Warning);
    pub const DANGER: Self = Self::Normal(SemanticColor::Danger);

    pub const PRIMARY_LIGHT: Self = Self::Light(SemanticColor::Primary);
    pub const LINK_LIGHT: Self = Self::Light(SemanticColor::Link);
    pub const INFO_LIGHT: Self = Self::Light(SemanticColor::Info);
    pub const SUCCESS_LIGHT: Self = Self::Light(SemanticColor::Success);
    pub const WARNING_LIGHT: Self = Self::Light(SemanticColor::Warning);
    pub const DANGER_LIGHT: Self = Self::Light(SemanticColor::Danger);
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TextShade {
    BlackBis,
    BlackTer,
    GreyDarker,
    GreyDark,
    Grey,
    GreyLight,
    GreyLighter,
    WhiteTer,
    WhiteBis,
}

impl TextShade {
    pub fn class(&self) -> &'static str {
        use TextShade::*;
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
