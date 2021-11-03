use yew::Callback;

pub use super::icon_kind::IconKind;
use crate::{helpers::Color, innerlude::*, Size};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct IconProps {
    #[prop_or_default]
    pub id: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub class: Classes,

    // TODO: implement
    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,

    /// Either an IconKind or a tuple (IconKind, IconKind) to indicate stacked
    /// icons
    pub kind: IconKind,
    // pub kind: IconStack,
    #[prop_or_default]
    pub color: Option<Color>,

    /// Container size
    #[prop_or_default]
    pub size: Option<Size>,

    /// Icon Size. Inferred from [`size`] if unset. You probably don't need this
    #[prop_or_default]
    pub icon_size: Option<IconSize>,

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

    #[prop_or_default]
    pub on_click: Option<Callback<web_sys::MouseEvent>>,
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

/// Font-Awesome Size
#[derive(Debug, Clone, Copy, PartialEq, Eq, BulmaClass)]
pub enum IconSize {
    /// `fa-xs: 0.75em`
    #[bulma_class = "fa-xs"]
    ExtraSmall,

    /// `fa-sm: 0.875em`
    #[bulma_class = "fa-sm"]
    Small,

    /// `fa-lg: 1.33em`
    #[bulma_class = "fa-lg"]
    Large,

    /// `fa-2x: 2em`
    #[bulma_class = "fa-2x"]
    X2,
    /// `fa-3x: 3em`
    #[bulma_class = "fa-3x"]
    X3,
    /// `fa-4x: 4em`
    #[bulma_class = "fa-4x"]
    X4,
    /// `fa-5x: 5em`
    #[bulma_class = "fa-5x"]
    X5,
    /// `fa-6x: 6em`
    #[bulma_class = "fa-6x"]
    X6,
    /// `fa-7x: 7em`
    #[bulma_class = "fa-7x"]
    X7,
    /// `fa-8x: 8em`
    #[bulma_class = "fa-8x"]
    X8,
    /// `fa-9x: 9em`
    #[bulma_class = "fa-9x"]
    X9,
    /// `fa-10x: 10em`
    #[bulma_class = "fa-10x"]
    X10,
}

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

#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    // TODO
    // match (self.props.rotate, self.props.flip) {
    //     (Some(rotate), Some(flip)) => {}

    //     (Some(rotate), None) => {}

    //     (None, Some(flip)) => {}

    //     (None, None) => {}
    // }

    // match self.props.kind {
    //     IconStack::Single(kind) => {}

    //     IconStack::Stacked { bottom, top } => {}
    // }

    let color = props.color.map(|c| c.text_class()).unwrap_or_default();

    let mut span_class = props.class.clone();
    unsafe {
        span_class.unchecked_push("icon");
        span_class.unchecked_push(color);
    }

    let mut i_class = classes!("fas");

    if let Some(animate) = &props.animate {
        match animate {
            IconAnimation::Spin => unsafe { i_class.unchecked_push("fa-spin") },
            IconAnimation::Pulse => unsafe { i_class.unchecked_push("fa-pulse") },
        }
    }

    if let Some(size) = &props.size {
        if !matches!(size, Size::Normal) {
            unsafe { span_class.unchecked_push(size.class()) }
        }
    }
    if let Some(size) = props.icon_size.or(props
        .size
        .as_ref()
        .map(|s| match s {
            Size::Small | Size::Normal => None,
            Size::Medium => Some(IconSize::Large),
            Size::Large => Some(IconSize::X2),
        })
        .flatten())
    {
        unsafe { i_class.unchecked_push(size.class()) }
    }

    // TODO: id on span vs text span?
    let icon = html! {
        <span id={props.id.clone()} class={span_class} onclick={props.on_click.clone()}>
            <i class={i_class}/>
        </span>
    };

    if let Some(text) = &props.text {
        html! {
            <span class={color}>
                <div>
                    <p>{icon}</p>
                    <p>{text}</p>
                </div>
            </span>
        }
    } else {
        icon
    }
}

// pub type IconText = Pure<PureIconText>;

// #[derive(Debug, Default, PartialEq, Clone, Properties)]
// pub struct PureIconText {
//     #[prop_or_default]
//     pub id: Option<Cow<'static, str>>,

//     #[prop_or_default]
//     pub class: Classes,

//     #[prop_or_default]
//     pub style: Option<String>,

//     #[prop_or_default]
//     pub inline: bool,

//     #[prop_or_default]
//     children: ChildrenWithProps<IconTextChild>,
// }

// #[derive(Debug, Clone)]
// pub enum IconTextChild {
//     Icon(Icon),
//     Text(String),
// }

// impl From<Icon> for IconTextChild {
//     fn from(icon: Icon) -> Self {
//         IconTextChild::Icon(icon)
//     }
// }

// impl From<String> for IconTextChild {
//     fn from(s: String) -> Self {
//         IconTextChild::Text(s)
//     }
// }

// impl From<IconTextChild> for Html {
//     fn from(child: IconTextChild) -> Self {
//         match child {
//             IconTextChild::Icon(icon) => icon.into(),
//             IconTextChild::Text(s) => s.into(),
//         }
//     }
// }
