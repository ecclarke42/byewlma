use crate::innerlude::*;

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct TitleProps {
    #[prop_or_default]
    pub id: Option<Cow<'static, str>>, // TODO: byewma macros ByewlmaProps for id, class, style, children?

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub level: Option<TitleLevel>,

    #[prop_or_default]
    pub spaced: bool,
}

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct SubtitleProps {
    #[prop_or_default]
    pub id: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub level: Option<TitleLevel>,
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

// TODO: tag based on Hx?

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

/// Bulma [Title](https://bulma.io/documentation/elements/title/) Element
#[function_component(Title)]
pub fn title(props: &TitleProps) -> Html {
    let mut class = props.class.clone();
    class.push("title");

    if let Some(level) = props.level {
        class.push(level.class());
    }

    if props.spaced {
        class.push("is-spaced");
    }

    html! {
        <div id={props.id.clone()} class={class} style={props.style.clone()}>
            {for props.children.iter()}
        </div>
    }
}

/// Bulma [Subtitle](https://bulma.io/documentation/elements/title/) Element
#[function_component(Subtitle)]
pub fn subtitle(props: &SubtitleProps) -> Html {
    let mut class = props.class.clone();
    class.push("subtitle");

    if let Some(level) = props.level {
        class.push(level.class());
    }

    html! {
        <div id={props.id.clone()} class={class} style={props.style.clone()}>
            {for props.children.iter()}
        </div>
    }
}
