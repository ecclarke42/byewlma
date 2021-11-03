use crate::{innerlude::*, Color};

#[derive(Debug, Default, PartialEq, Properties)]
pub struct TextProps {
    #[prop_or_default]
    pub id: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub tag: Option<&'static str>,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub size: Option<TextSize>,
    /// Set size when viewport is less than the [`$tablet`](https://bulma.io/documentation/customize/variables/) breakpoint (default: `769px`)
    #[prop_or_default]
    pub size_mobile: Option<TextSize>,
    /// Set size when viewport is less than the [`$desktop`](https://bulma.io/documentation/customize/variables/) breakpoint (default: `1023px`)
    #[prop_or_default]
    pub size_touch: Option<TextSize>,
    /// Set size when viewport is at least the [`$tablet`](https://bulma.io/documentation/customize/variables/) breakpoint (default: `769px`)
    #[prop_or_default]
    pub size_tablet_and_greater: Option<TextSize>,
    /// Set size when viewport is at least the [`$desktop`](https://bulma.io/documentation/customize/variables/) breakpoint (default: `1024px`)
    #[prop_or_default]
    pub size_desktop_and_greater: Option<TextSize>,
    /// Set size when viewport is at least the [`$widescreen`](https://bulma.io/documentation/customize/variables/) breakpoint (default: `1216px`)
    #[prop_or_default]
    pub size_widescreen_and_greater: Option<TextSize>,
    /// Set size when viewport is at least the [`$fullhd`](https://bulma.io/documentation/customize/variables/) breakpoint (default: `1408px`)
    #[prop_or_default]
    pub size_fullhd_and_greater: Option<TextSize>,

    #[prop_or_default]
    pub alignment: Option<TextAlignment>,
    /// Set alignment when viewport is less than the [`$tablet`](https://bulma.io/documentation/customize/variables/) breakpoint (default: `769px`)
    #[prop_or_default]
    pub alignment_mobile: Option<TextAlignment>,
    /// Set alignment when viewport is less than the [`$desktop`](https://bulma.io/documentation/customize/variables/) breakpoint (default: `1023px`)
    #[prop_or_default]
    pub alignment_touch: Option<TextAlignment>,
    /// Set alignment when viewport is at least the [`$tablet`](https://bulma.io/documentation/customize/variables/) breakpoint (default: `769px`)
    #[prop_or_default]
    pub alignment_tablet_and_greater: Option<TextAlignment>,
    /// Set alignment when viewport is at least the [`$desktop`](https://bulma.io/documentation/customize/variables/) breakpoint (default: `1024px`)
    #[prop_or_default]
    pub alignment_desktop_and_greater: Option<TextAlignment>,
    /// Set alignment when viewport is at least the [`$widescreen`](https://bulma.io/documentation/customize/variables/) breakpoint (default: `1216px`)
    #[prop_or_default]
    pub alignment_widescreen_and_greater: Option<TextAlignment>,
    /// Set alignment when viewport is at least the [`$fullhd`](https://bulma.io/documentation/customize/variables/) breakpoint (default: `1408px`)
    #[prop_or_default]
    pub alignment_fullhd_and_greater: Option<TextAlignment>,

    /// Set alignment when viewport is in the tablet range, between the
    /// [`$tablet`](https://bulma.io/documentation/customize/variables/) (default: `769px`)
    /// and
    /// [`$desktop`](https://bulma.io/documentation/customize/variables/) (default: `1023px`)
    /// breakpoints
    #[prop_or_default]
    pub alignment_only_tablet: Option<TextAlignment>,

    /// Set alignment when viewport is in the tablet range, between the
    /// [`$desktop`](https://bulma.io/documentation/customize/variables/) (default: `1023px`)
    /// and
    /// [`$widescreen`](https://bulma.io/documentation/customize/variables/) (default: `1216px`)
    /// breakpoints
    #[prop_or_default]
    pub alignment_only_desktop: Option<TextAlignment>,

    /// Set alignment when viewport is in the tablet range, between the
    /// [`$widescreen`](https://bulma.io/documentation/customize/variables/) (default: `1216px`)
    /// and
    /// [`$fullhd`](https://bulma.io/documentation/customize/variables/) (default: `1408px`)
    /// breakpoints
    #[prop_or_default]
    pub alignment_only_widescreen: Option<TextAlignment>,

    #[prop_or_default]
    pub transform: Option<TextTransform>,

    #[prop_or_default]
    pub weight: Option<TextWeight>,

    #[prop_or_default]
    pub font_family: Option<FontFamily>,
}

#[derive(Debug, Clone, Copy, PartialEq, BulmaClass)]
pub enum TextSize {
    /// `font-size:`[`$size-1`](https://bulma.io/documentation/customize/variables/) (default: `3rem`)
    #[bulma_class = "is-size-1"]
    Sz1,
    /// `font-size:`[`$size-2`](https://bulma.io/documentation/customize/variables/) (default: `2.5rem`)
    #[bulma_class = "is-size-2"]
    Sz2,
    /// `font-size:`[`$size-3`](https://bulma.io/documentation/customize/variables/) (default: `2rem`)
    #[bulma_class = "is-size-3"]
    Sz3,
    /// `font-size:`[`$size-4`](https://bulma.io/documentation/customize/variables/) (default: `1.5rem`)
    #[bulma_class = "is-size-4"]
    Sz4,
    /// `font-size:`[`$size-5`](https://bulma.io/documentation/customize/variables/) (default: `1.25rem`)
    #[bulma_class = "is-size-5"]
    Sz5,
    /// `font-size:`[`$size-6`](https://bulma.io/documentation/customize/variables/) (default: `1rem`)
    #[bulma_class = "is-size-6"]
    Sz6,
    /// `font-size:`[`$size-7`](https://bulma.io/documentation/customize/variables/) (default: `0.75rem`)
    #[bulma_class = "is-size-7"]
    Sz7,
}

impl TextSize {
    pub fn mobile(&self) -> &'static str {
        match self {
            TextSize::Sz1 => "is-size-1-mobile",
            TextSize::Sz2 => "is-size-2-mobile",
            TextSize::Sz3 => "is-size-3-mobile",
            TextSize::Sz4 => "is-size-4-mobile",
            TextSize::Sz5 => "is-size-5-mobile",
            TextSize::Sz6 => "is-size-6-mobile",
            TextSize::Sz7 => "is-size-7-mobile",
        }
    }
    pub fn touch(&self) -> &'static str {
        match self {
            TextSize::Sz1 => "is-size-1-touch",
            TextSize::Sz2 => "is-size-2-touch",
            TextSize::Sz3 => "is-size-3-touch",
            TextSize::Sz4 => "is-size-4-touch",
            TextSize::Sz5 => "is-size-5-touch",
            TextSize::Sz6 => "is-size-6-touch",
            TextSize::Sz7 => "is-size-7-touch",
        }
    }
    pub fn tablet(&self) -> &'static str {
        match self {
            TextSize::Sz1 => "is-size-1-tablet",
            TextSize::Sz2 => "is-size-2-tablet",
            TextSize::Sz3 => "is-size-3-tablet",
            TextSize::Sz4 => "is-size-4-tablet",
            TextSize::Sz5 => "is-size-5-tablet",
            TextSize::Sz6 => "is-size-6-tablet",
            TextSize::Sz7 => "is-size-7-tablet",
        }
    }
    pub fn desktop(&self) -> &'static str {
        match self {
            TextSize::Sz1 => "is-size-1-desktop",
            TextSize::Sz2 => "is-size-2-desktop",
            TextSize::Sz3 => "is-size-3-desktop",
            TextSize::Sz4 => "is-size-4-desktop",
            TextSize::Sz5 => "is-size-5-desktop",
            TextSize::Sz6 => "is-size-6-desktop",
            TextSize::Sz7 => "is-size-7-desktop",
        }
    }
    pub fn widescreen(&self) -> &'static str {
        match self {
            TextSize::Sz1 => "is-size-1-widescreen",
            TextSize::Sz2 => "is-size-2-widescreen",
            TextSize::Sz3 => "is-size-3-widescreen",
            TextSize::Sz4 => "is-size-4-widescreen",
            TextSize::Sz5 => "is-size-5-widescreen",
            TextSize::Sz6 => "is-size-6-widescreen",
            TextSize::Sz7 => "is-size-7-widescreen",
        }
    }
    pub fn fullhd(&self) -> &'static str {
        match self {
            TextSize::Sz1 => "is-size-1-fullhd",
            TextSize::Sz2 => "is-size-2-fullhd",
            TextSize::Sz3 => "is-size-3-fullhd",
            TextSize::Sz4 => "is-size-4-fullhd",
            TextSize::Sz5 => "is-size-5-fullhd",
            TextSize::Sz6 => "is-size-6-fullhd",
            TextSize::Sz7 => "is-size-7-fullhd",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, BulmaClass)]
pub enum TextAlignment {
    #[bulma_class = "has-text-centered"]
    Centered,
    #[bulma_class = "has-text-justified"]
    Justified,
    #[bulma_class = "has-text-left"]
    Left,
    #[bulma_class = "has-text-right"]
    Right,
}

impl TextAlignment {
    pub fn mobile(&self) -> &'static str {
        match self {
            TextAlignment::Centered => "has-text-centered-mobile",
            TextAlignment::Justified => "has-text-justified-mobile",
            TextAlignment::Left => "has-text-left-mobile",
            TextAlignment::Right => "has-text-right-mobile",
        }
    }
    pub fn touch(&self) -> &'static str {
        match self {
            TextAlignment::Centered => "has-text-centered-touch",
            TextAlignment::Justified => "has-text-justified-touch",
            TextAlignment::Left => "has-text-left-touch",
            TextAlignment::Right => "has-text-right-touch",
        }
    }
    pub fn tablet_only(&self) -> &'static str {
        match self {
            TextAlignment::Centered => "has-text-centered-tablet-only",
            TextAlignment::Justified => "has-text-justified-tablet-only",
            TextAlignment::Left => "has-text-left-tablet-only",
            TextAlignment::Right => "has-text-right-tablet-only",
        }
    }
    pub fn tablet(&self) -> &'static str {
        match self {
            TextAlignment::Centered => "has-text-centered-tablet",
            TextAlignment::Justified => "has-text-justified-tablet",
            TextAlignment::Left => "has-text-left-tablet",
            TextAlignment::Right => "has-text-right-tablet",
        }
    }
    pub fn desktop_only(&self) -> &'static str {
        match self {
            TextAlignment::Centered => "has-text-centered-desktop-only",
            TextAlignment::Justified => "has-text-justified-desktop-only",
            TextAlignment::Left => "has-text-left-desktop-only",
            TextAlignment::Right => "has-text-right-desktop-only",
        }
    }
    pub fn desktop(&self) -> &'static str {
        match self {
            TextAlignment::Centered => "has-text-centered-desktop",
            TextAlignment::Justified => "has-text-justified-desktop",
            TextAlignment::Left => "has-text-left-desktop",
            TextAlignment::Right => "has-text-right-desktop",
        }
    }
    pub fn widescreen_only(&self) -> &'static str {
        match self {
            TextAlignment::Centered => "has-text-centered-widescreen-only",
            TextAlignment::Justified => "has-text-justified-widescreen-only",
            TextAlignment::Left => "has-text-left-widescreen-only",
            TextAlignment::Right => "has-text-right-widescreen-only",
        }
    }
    pub fn widescreen(&self) -> &'static str {
        match self {
            TextAlignment::Centered => "has-text-centered-widescreen",
            TextAlignment::Justified => "has-text-justified-widescreen",
            TextAlignment::Left => "has-text-left-widescreen",
            TextAlignment::Right => "has-text-right-widescreen",
        }
    }
    pub fn fullhd(&self) -> &'static str {
        match self {
            TextAlignment::Centered => "has-text-centered-fullhd",
            TextAlignment::Justified => "has-text-justified-fullhd",
            TextAlignment::Left => "has-text-left-fullhd",
            TextAlignment::Right => "has-text-right-fullhd",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, BulmaClass)]
pub enum TextTransform {
    #[bulma_class = "is-capitalized"]
    Capitalize,
    #[bulma_class = "is-lowercase"]
    Lowercase,
    #[bulma_class = "is-uppercase"]
    Uppercase,
    #[bulma_class = "is-italic"]
    Italic,
}

#[derive(Debug, Clone, Copy, PartialEq, BulmaClass)]
pub enum TextWeight {
    #[bulma_class = "has-text-weight-light"]
    Light,
    #[bulma_class = "has-text-weight-normal"]
    Normal,
    #[bulma_class = "has-text-weight-medium"]
    Medium,
    #[bulma_class = "has-text-weight-semibold"]
    SemiBold,
    #[bulma_class = "has-text-weight-bold"]
    Bold,
}

#[derive(Debug, Clone, Copy, PartialEq, BulmaClass)]
pub enum FontFamily {
    /// Set family to [`$family-sans-serif`](https://bulma.io/documentation/customize/variables/)
    #[bulma_class = "is-family-sans-serif"]
    SansSerif,
    /// Set family to [`$family-monospace`](https://bulma.io/documentation/customize/variables/)
    #[bulma_class = "is-family-monospace"]
    Monospace,
    /// Set family to [`$family-primary`](https://bulma.io/documentation/customize/variables/),
    /// which is, by default, equal to [`$family-sans-serif`](https://bulma.io/documentation/customize/variables/)
    #[bulma_class = "is-family-primary"]
    Primary,
    /// Set family to [`$family-secondary`](https://bulma.io/documentation/customize/variables/),
    /// which is, by default, equal to [`$family-sans-serif`](https://bulma.io/documentation/customize/variables/)
    #[bulma_class = "is-family-secondary"]
    Secondary,
    /// Set family to [`$family-code`](https://bulma.io/documentation/customize/variables/)
    /// which is, by default, equal to [`$family-monospace`](https://bulma.io/documentation/customize/variables/)
    #[bulma_class = "is-family-code"]
    Code,
}

/// Bulma Typography Helper
///
/// Renders as a <span/>, by default, but can be overidden by the `tag` property
#[function_component(Text)]
pub fn text(props: &TextProps) -> Html {
    let mut class = props.class.clone();

    if let Some(color) = &props.color {
        class.push(color.text_class());
    }

    if let Some(size) = &props.size {
        class.add(size);
    }

    unsafe {
        if let Some(size) = &props.size_mobile {
            class.unchecked_push(size.mobile());
        }
        if let Some(size) = &props.size_touch {
            class.unchecked_push(size.touch());
        }
        if let Some(size) = &props.size_tablet_and_greater {
            class.unchecked_push(size.tablet());
        }
        if let Some(size) = &props.size_desktop_and_greater {
            class.unchecked_push(size.desktop());
        }
        if let Some(size) = &props.size_widescreen_and_greater {
            class.unchecked_push(size.widescreen());
        }
        if let Some(size) = &props.size_fullhd_and_greater {
            class.unchecked_push(size.fullhd());
        }
    }

    if let Some(alignment) = &props.alignment {
        class.add(alignment);
    }

    unsafe {
        if let Some(alignment) = &props.alignment_mobile {
            class.unchecked_push(alignment.mobile());
        }
        if let Some(alignment) = &props.alignment_touch {
            class.unchecked_push(alignment.touch());
        }
        if let Some(alignment) = &props.alignment_only_tablet {
            class.unchecked_push(alignment.tablet_only());
        }
        if let Some(alignment) = &props.alignment_tablet_and_greater {
            class.unchecked_push(alignment.tablet());
        }
        if let Some(alignment) = &props.alignment_only_desktop {
            class.unchecked_push(alignment.desktop_only());
        }
        if let Some(alignment) = &props.alignment_desktop_and_greater {
            class.unchecked_push(alignment.desktop());
        }
        if let Some(alignment) = &props.alignment_only_widescreen {
            class.unchecked_push(alignment.widescreen_only());
        }
        if let Some(alignment) = &props.alignment_widescreen_and_greater {
            class.unchecked_push(alignment.widescreen());
        }
        if let Some(alignment) = &props.alignment_fullhd_and_greater {
            class.unchecked_push(alignment.fullhd());
        }
    }

    if let Some(transform) = &props.transform {
        class.add(transform);
    }

    if let Some(weight) = &props.weight {
        class.add(weight);
    }

    if let Some(family) = &props.font_family {
        class.add(family);
    }

    html! {
        <@{props.tag.unwrap_or("span")} id={props.id.clone()} class={class} style={props.style.clone()}>
            {for props.children.iter()}
        </@>
    }
}
