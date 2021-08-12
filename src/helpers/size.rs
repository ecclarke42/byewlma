use yew::Classes;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size {
    Small,
    Normal,
    Medium,
    Large,
}

impl Size {
    pub fn class(&self) -> &'static str {
        use Size::*;
        match self {
            Small => "is-small",
            Normal => "is-normal",
            Medium => "is-medium",
            Large => "is-large",
        }
    }

    pub fn add_class(&self, classes: &mut Classes) {
        classes.push(self.class())// TODO unchecked push?
    }
}

impl From<Size> for Classes {
    fn from(size: Size) -> Self {
        Classes::from(size.class())
    }
}

impl From<&Size> for Classes {
    fn from(size: &Size) -> Self {
        Classes::from(size.class())
    }
}
