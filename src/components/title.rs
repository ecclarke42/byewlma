use crate::components::prelude::*;

pure_props! {
    /// Bulma [Title](https://bulma.io/documentation/elements/title/) Element
    pub struct Title {
        #[prop_or_default]
        pub level: Option<TitleLevel>,

        #[prop_or_default]
        pub spaced: bool,
    }
}

pure_props! {
    /// Bulma [Subtitle](https://bulma.io/documentation/elements/title/) Element
    pub struct Subtitle {
        #[prop_or_default]
        pub level: Option<TitleLevel>,
    }
}

pub type Level = TitleLevel;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TitleLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl TitleLevel {
    pub fn class(&self) -> &'static str {
        match self {
            TitleLevel::H1 => "is-1",
            TitleLevel::H2 => "is-2",
            TitleLevel::H3 => "is-3",
            TitleLevel::H4 => "is-4",
            TitleLevel::H5 => "is-5",
            TitleLevel::H6 => "is-6",
        }
    }
}

impl PureComponent for PureTitle {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        class.push("title");

        if let Some(level) = self.level {
            class.push(level.class());
        }

        if self.spaced {
            class.push("is-spaced");
        }

        html! {
            <div id={self.id.clone()} class={class} style={self.style.clone()}>
                {for self.children.iter()}
            </div>
        }
    }
}

impl PureComponent for PureSubtitle {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        class.push("subtitle");

        if let Some(level) = self.level {
            class.push(level.class());
        }

        html! {
            <div id={self.id.clone()} class={class} style={self.style.clone()}>
                {for self.children.iter()}
            </div>
        }
    }
}
