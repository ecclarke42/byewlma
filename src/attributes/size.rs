#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Size {
    Small,
    Default,
    Medium,
    Large,
}

impl Size {
    pub fn class(&self) -> &'static str {
        match self {
            Size::Small => "is-small",
            Size::Default => "",
            Size::Medium => "is-medium",
            Size::Large => "is-large",
        }
    }

    pub fn fa_class(&self) -> &'static str {
        match self {
            Size::Small => "fas",
            Size::Default => "fas",
            Size::Medium => "fas fa-lg",
            Size::Large => "fas fa-2x",
        }
    }
}
