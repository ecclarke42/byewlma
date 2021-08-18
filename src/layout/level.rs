use crate::components::prelude::*;
use std::borrow::Cow;
use yew::html::ChildrenRenderer;

// TODO: doesn't seem like this works for pureComponents with current Pure<T>

/// Bulma [Level](https://bulma.io/documentation/layout/level/) Element
pub type SplitLevel = Pure<PureSplitLevel>;

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct PureSplitLevel {
    #[prop_or_default]
    pub id: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub children: ChildrenRenderer<LevelChild>,
}

impl PureComponent for PureSplitLevel {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        unsafe {
            class.unchecked_push("level");
        }

        // TODO: collect children into two?
        // self.children.into_iter().map(|child| {
        // match
        // })

        html! {
            <div id={self.id.clone()} class={class} style={self.style.clone()}>
                {for self.children.iter()}
            </div>
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LevelChild {
    Left(LevelLeft),
    Right(LevelRight),
}

pure_props! {
    /// A wrapping element for the left side of a Level.
    ///
    /// Each child will be wrapped in a level-item div
    pub struct LevelLeft {}
}
impl PureComponent for PureLevelLeft {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        unsafe {
            class.unchecked_push("level-left");
        }
        html! {
            <div id={self.id.clone()} class={class} style={self.style.clone()}>
                {for self.children.iter().map(|child| html! {
                    <div class="level-item">
                        {child}
                    </div>
                })}
            </div>
        }
    }
}
pure_props! {
    /// A wrapping element for the left side of a Level.
    ///
    /// Each child will be wrapped in a level-item div
    pub struct LevelRight {}
}
impl PureComponent for PureLevelRight {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        unsafe {
            class.unchecked_push("level-right");
        }
        html! {
            <div id={self.id.clone()} class={class} style={self.style.clone()}>
                {for self.children.iter().map(|child| html! {
                    <div class="level-item">
                        {child}
                    </div>
                })}
            </div>
        }
    }
}

impl From<LevelLeft> for LevelChild {
    fn from(child: LevelLeft) -> Self {
        LevelChild::Left(child)
    }
}
impl From<LevelRight> for LevelChild {
    fn from(child: LevelRight) -> Self {
        LevelChild::Right(child)
    }
}
impl From<LevelChild> for Html {
    fn from(child: LevelChild) -> Self {
        match child {
            LevelChild::Left(child) => child.into(),
            LevelChild::Right(child) => child.into(),
        }
    }
}

pure_props! {
    /// Bulma [Level](https://bulma.io/documentation/layout/level/) Element
    /// with centered children
    pub struct CenteredLevel {}
}
impl PureComponent for PureCenteredLevel {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        unsafe {
            class.unchecked_push("level");
        }
        html! {
            <div id={self.id.clone()} class={class} style={self.style.clone()}>
                {for self.children.iter().map(|child| html! {
                    <div class="level-item has-text-centered">
                        {child}
                    </div>
                })}
            </div>
        }
    }
}
