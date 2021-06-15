use crate::components::prelude::*;

pure_props! {
    /// Bulma Typography Helper
    ///
    /// Renders as a <span/>, by default, but can be overidden by the `tag` property
    pub struct Text {
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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextSize {
    /// `font-size:`[`$size-1`](https://bulma.io/documentation/customize/variables/) (default: `3rem`)
    Sz1,
    /// `font-size:`[`$size-2`](https://bulma.io/documentation/customize/variables/) (default: `2.5rem`)
    Sz2,
    /// `font-size:`[`$size-3`](https://bulma.io/documentation/customize/variables/) (default: `2rem`)
    Sz3,
    /// `font-size:`[`$size-4`](https://bulma.io/documentation/customize/variables/) (default: `1.5rem`)
    Sz4,
    /// `font-size:`[`$size-5`](https://bulma.io/documentation/customize/variables/) (default: `1.25rem`)
    Sz5,
    /// `font-size:`[`$size-6`](https://bulma.io/documentation/customize/variables/) (default: `1rem`)
    Sz6,
    /// `font-size:`[`$size-7`](https://bulma.io/documentation/customize/variables/) (default: `0.75rem`)
    Sz7,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextAlignment {
    Centered,
    Justified,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextTransform {
    Capitalize,
    Lowercase,
    Uppercase,
    Italic,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextWeight {
    Light,
    Normal,
    Medium,
    SemiBold,
    Bold,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontFamily {
    /// Set family to [`$family-sans-serif`](https://bulma.io/documentation/customize/variables/)
    SansSerif,
    /// Set family to [`$family-monospace`](https://bulma.io/documentation/customize/variables/)
    Monospace,
    /// Set family to [`$family-primary`](https://bulma.io/documentation/customize/variables/),
    /// which is, by default, equal to [`$family-sans-serif`](https://bulma.io/documentation/customize/variables/)
    Primary,
    /// Set family to [`$family-secondary`](https://bulma.io/documentation/customize/variables/),
    /// which is, by default, equal to [`$family-sans-serif`](https://bulma.io/documentation/customize/variables/)
    Secondary,
    /// Set family to [`$family-code`](https://bulma.io/documentation/customize/variables/)
    /// which is, by default, equal to [`$family-monospace`](https://bulma.io/documentation/customize/variables/)
    Code,
}

impl PureComponent for PureText {
    fn render(&self) -> Html {
        let mut class = self.class.clone();

        if let Some(color) = self.color {
            class.push(color.text_class());
        }

        if let Some(size) = self.size {
            class.push(match size {
                TextSize::Sz1 => "is-size-1",
                TextSize::Sz2 => "is-size-2",
                TextSize::Sz3 => "is-size-3",
                TextSize::Sz4 => "is-size-4",
                TextSize::Sz5 => "is-size-5",
                TextSize::Sz6 => "is-size-6",
                TextSize::Sz7 => "is-size-7",
            });
        }

        if let Some(size) = self.size_mobile {
            class.push(match size {
                TextSize::Sz1 => "is-size-1-mobile",
                TextSize::Sz2 => "is-size-2-mobile",
                TextSize::Sz3 => "is-size-3-mobile",
                TextSize::Sz4 => "is-size-4-mobile",
                TextSize::Sz5 => "is-size-5-mobile",
                TextSize::Sz6 => "is-size-6-mobile",
                TextSize::Sz7 => "is-size-7-mobile",
            });
        }

        if let Some(size) = self.size_touch {
            class.push(match size {
                TextSize::Sz1 => "is-size-1-touch",
                TextSize::Sz2 => "is-size-2-touch",
                TextSize::Sz3 => "is-size-3-touch",
                TextSize::Sz4 => "is-size-4-touch",
                TextSize::Sz5 => "is-size-5-touch",
                TextSize::Sz6 => "is-size-6-touch",
                TextSize::Sz7 => "is-size-7-touch",
            });
        }

        if let Some(size) = self.size_tablet_and_greater {
            class.push(match size {
                TextSize::Sz1 => "is-size-1-tablet",
                TextSize::Sz2 => "is-size-2-tablet",
                TextSize::Sz3 => "is-size-3-tablet",
                TextSize::Sz4 => "is-size-4-tablet",
                TextSize::Sz5 => "is-size-5-tablet",
                TextSize::Sz6 => "is-size-6-tablet",
                TextSize::Sz7 => "is-size-7-tablet",
            });
        }

        if let Some(size) = self.size_desktop_and_greater {
            class.push(match size {
                TextSize::Sz1 => "is-size-1-desktop",
                TextSize::Sz2 => "is-size-2-desktop",
                TextSize::Sz3 => "is-size-3-desktop",
                TextSize::Sz4 => "is-size-4-desktop",
                TextSize::Sz5 => "is-size-5-desktop",
                TextSize::Sz6 => "is-size-6-desktop",
                TextSize::Sz7 => "is-size-7-desktop",
            });
        }

        if let Some(size) = self.size_widescreen_and_greater {
            class.push(match size {
                TextSize::Sz1 => "is-size-1-widescreen",
                TextSize::Sz2 => "is-size-2-widescreen",
                TextSize::Sz3 => "is-size-3-widescreen",
                TextSize::Sz4 => "is-size-4-widescreen",
                TextSize::Sz5 => "is-size-5-widescreen",
                TextSize::Sz6 => "is-size-6-widescreen",
                TextSize::Sz7 => "is-size-7-widescreen",
            });
        }

        if let Some(size) = self.size_fullhd_and_greater {
            class.push(match size {
                TextSize::Sz1 => "is-size-1-fullhd",
                TextSize::Sz2 => "is-size-2-fullhd",
                TextSize::Sz3 => "is-size-3-fullhd",
                TextSize::Sz4 => "is-size-4-fullhd",
                TextSize::Sz5 => "is-size-5-fullhd",
                TextSize::Sz6 => "is-size-6-fullhd",
                TextSize::Sz7 => "is-size-7-fullhd",
            });
        }

        if let Some(alignment) = self.alignment {
            class.push(match alignment {
                TextAlignment::Centered => "has-text-centered",
                TextAlignment::Justified => "has-text-justified",
                TextAlignment::Left => "has-text-left",
                TextAlignment::Right => "has-text-right",
            });
        }

        if let Some(alignment) = self.alignment_mobile {
            class.push(match alignment {
                TextAlignment::Centered => "has-text-centered-mobile",
                TextAlignment::Justified => "has-text-justified-mobile",
                TextAlignment::Left => "has-text-left-mobile",
                TextAlignment::Right => "has-text-right-mobile",
            });
        }

        if let Some(alignment) = self.alignment_touch {
            class.push(match alignment {
                TextAlignment::Centered => "has-text-centered-touch",
                TextAlignment::Justified => "has-text-justified-touch",
                TextAlignment::Left => "has-text-left-touch",
                TextAlignment::Right => "has-text-right-touch",
            });
        }

        if let Some(alignment) = self.alignment_only_tablet {
            class.push(match alignment {
                TextAlignment::Centered => "has-text-centered-tablet-only",
                TextAlignment::Justified => "has-text-justified-tablet-only",
                TextAlignment::Left => "has-text-left-tablet-only",
                TextAlignment::Right => "has-text-right-tablet-only",
            });
        }

        if let Some(alignment) = self.alignment_tablet_and_greater {
            class.push(match alignment {
                TextAlignment::Centered => "has-text-centered-tablet",
                TextAlignment::Justified => "has-text-justified-tablet",
                TextAlignment::Left => "has-text-left-tablet",
                TextAlignment::Right => "has-text-right-tablet",
            });
        }

        if let Some(alignment) = self.alignment_only_desktop {
            class.push(match alignment {
                TextAlignment::Centered => "has-text-centered-desktop-only",
                TextAlignment::Justified => "has-text-justified-desktop-only",
                TextAlignment::Left => "has-text-left-desktop-only",
                TextAlignment::Right => "has-text-right-desktop-only",
            });
        }

        if let Some(alignment) = self.alignment_desktop_and_greater {
            class.push(match alignment {
                TextAlignment::Centered => "has-text-centered-desktop",
                TextAlignment::Justified => "has-text-justified-desktop",
                TextAlignment::Left => "has-text-left-desktop",
                TextAlignment::Right => "has-text-right-desktop",
            });
        }

        if let Some(alignment) = self.alignment_only_widescreen {
            class.push(match alignment {
                TextAlignment::Centered => "has-text-centered-widescreen-only",
                TextAlignment::Justified => "has-text-justified-widescreen-only",
                TextAlignment::Left => "has-text-left-widescreen-only",
                TextAlignment::Right => "has-text-right-widescreen-only",
            });
        }

        if let Some(alignment) = self.alignment_widescreen_and_greater {
            class.push(match alignment {
                TextAlignment::Centered => "has-text-centered-widescreen",
                TextAlignment::Justified => "has-text-justified-widescreen",
                TextAlignment::Left => "has-text-left-widescreen",
                TextAlignment::Right => "has-text-right-widescreen",
            });
        }

        if let Some(alignment) = self.alignment_fullhd_and_greater {
            class.push(match alignment {
                TextAlignment::Centered => "has-text-centered-fullhd",
                TextAlignment::Justified => "has-text-justified-fullhd",
                TextAlignment::Left => "has-text-left-fullhd",
                TextAlignment::Right => "has-text-right-fullhd",
            });
        }

        if let Some(transform) = self.transform {
            class.push(match transform {
                TextTransform::Capitalize => "is-capitalized",
                TextTransform::Lowercase => "is-lowercase",
                TextTransform::Uppercase => "is-uppercase",
                TextTransform::Italic => "is-italic",
            })
        }

        if let Some(transform) = self.transform {
            class.push(match transform {
                TextTransform::Capitalize => "is-capitalized",
                TextTransform::Lowercase => "is-lowercase",
                TextTransform::Uppercase => "is-uppercase",
                TextTransform::Italic => "is-italic",
            })
        }

        if let Some(weight) = self.weight {
            class.push(match weight {
                TextWeight::Light => "has-text-weight-light",
                TextWeight::Normal => "has-text-weight-normal",
                TextWeight::Medium => "has-text-weight-medium",
                TextWeight::SemiBold => "has-text-weight-semibold",
                TextWeight::Bold => "has-text-weight-bold",
            })
        }

        if let Some(family) = self.font_family {
            class.push(match family {
                FontFamily::SansSerif => "is-family-sans-serif",
                FontFamily::Monospace => "is-family-monospace",
                FontFamily::Primary => "is-family-primary",
                FontFamily::Secondary => "is-family-secondary",
                FontFamily::Code => "is-family-code",
            })
        }

        html! {
            <@{self.tag.unwrap_or("span")} id={self.id.clone()} class={class} style={self.style.clone()}>
                {for self.children.iter()}
            </@>
        }
    }
}
