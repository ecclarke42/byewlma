use crate::components::prelude::*;
use crate::helpers::Color;
use std::convert::AsRef;
use strum::AsRefStr;

pub use super::icon_kind::IconKind;

pub struct Icon {
    props: IconProps,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct IconProps {
    /// Either an IconKind or a tuple (IconKind, IconKind) to indicate stacked icons
    pub kind: IconStack,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub text: Option<String>,

    #[prop_or_default]
    pub fixed_width: bool,

    /// Rotate the icon
    ///
    /// Note, if rotate and flip are both specified, flip will be applied FIRST
    #[prop_or_default]
    pub rotate: Option<IconRotation>,

    /// Flip the icon
    ///
    /// Note, if rotate and flip are both specified, flip will be applied FIRST
    #[prop_or_default]
    pub flip: Option<IconFlip>,

    #[prop_or_default]
    pub animate: Option<IconAnimation>,

    #[prop_or_default]
    pub pull: Option<IconPull>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconStack {
    Single(IconKind),
    Stacked { bottom: IconKind, top: IconKind },
}

impl yew::html::IntoPropValue<IconStack> for IconKind {
    fn into_prop_value(self) -> IconStack {
        IconStack::Single(self)
    }
}

impl yew::html::IntoPropValue<IconStack> for (IconKind, IconKind) {
    fn into_prop_value(self) -> IconStack {
        IconStack::Stacked {
            bottom: self.0,
            top: self.1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconSize {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconRotation {
    CW90,
    CW180,
    CW270,

    CCW90,
    CCW180,
    CCW270,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconFlip {
    Horizontal,
    Vertical,
    Both,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconAnimation {
    /// Rotate smoothly
    Spin,

    /// Rotate in 8 steps
    Pulse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconPull {
    Left,
    Right,
}

impl Component for Icon {
    type Properties = IconProps;
    type Message = ();

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // match (self.props.rotate, self.props.flip) {
        //     (Some(rotate), Some(flip)) => {}

        //     (Some(rotate), None) => {}

        //     (None, Some(flip)) => {}

        //     (None, None) => {}
        // }

        match self.props.kind {
            IconStack::Single(kind) => {}

            IconStack::Stacked { bottom, top } => {}
        }

        if let Some(text) = &self.props.text {
            html! {}
        } else {
            html! {}
        }
    }
}
