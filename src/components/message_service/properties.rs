#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Default,
    Dark,
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,
}

impl Color {
    pub(crate) fn class(&self) -> Option<&'static str> {
        match self {
            Color::Default => None,
            Color::Dark => Some("is-dark"),
            Color::Primary => Some("is-primary"),
            Color::Link => Some("is-link"),
            Color::Info => Some("is-info"),
            Color::Success => Some("is-success"),
            Color::Warning => Some("is-warning"),
            Color::Danger => Some("is-danger"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Size {
    Small,
    Normal,
    Medium,
    Large,
}

impl Size {
    pub(crate) fn class(&self) -> Option<&'static str> {
        match self {
            Size::Small => Some("is-small"),
            Size::Normal => None,
            Size::Medium => Some("is-medium"),
            Size::Large => Some("is-large"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Position {
    TopLeft,
    // TopMiddle,
    TopRight,

    BottomLeft,
    // BottomMiddle,
    BottomRight,
}

impl Position {
    pub(crate) fn style(&self) -> &'static str {
        match self {
            Position::TopLeft => "byewlma-msg-svc-top-left",
            Position::TopRight => "byewlma-msg-svc-top-right",
            Position::BottomLeft => "byewlma-msg-svc-bottom-left",
            Position::BottomRight => "byewlma-msg-svc-bottom-right",
        }
    }

    pub(crate) fn animate_in_style(&self) -> &'static str {
        match self {
            Position::TopLeft => "byewlma-msg-svc-animate-in-from-left",
            Position::TopRight => "byewlma-msg-svc-animate-in-from-right",
            Position::BottomLeft => "byewlma-msg-svc-animate-in-from-left",
            Position::BottomRight => "byewlma-msg-svc-animate-in-from-right",
        }
    }

    pub(crate) fn animate_out_style(&self) -> &'static str {
        match self {
            Position::TopLeft => "byewlma-msg-svc-animate-out-to-left",
            Position::TopRight => "byewlma-msg-svc-animate-out-to-right",
            Position::BottomLeft => "byewlma-msg-svc-animate-out-to-left",
            Position::BottomRight => "byewlma-msg-svc-animate-out-to-right",
        }
    }
}
