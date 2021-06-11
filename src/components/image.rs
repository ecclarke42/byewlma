use crate::components::prelude::*;

/// Bulma [Image](https://bulma.io/documentation/elements/image/) Element
pub type Image = Pure<PureImage>;

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct PureImage {
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<String>,

    pub src: String,

    #[prop_or_default]
    pub size: Option<ImageSize>,

    #[prop_or_default]
    pub rounded: bool,

    #[prop_or_default]
    pub fullwidth: bool,
}

/// Image sizes, either in exact pixels (::Pixels*) or ratio (::Ratio*).
///
/// Note: When using Ratio.. variants, the parent element must have a specific
/// width.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImageSize {
    Pixels16x16,
    Pixels24x24,
    Pixels32x32,
    Pixels48x48,
    Pixels64x64,
    Pixels96x96,
    Pixels128x128,

    RatioSquare,
    Ratio1x1,
    Ratio5x4,
    Ratio4x3,
    Ratio3x2,
    Ratio5x3,
    Ratio16x9,
    Ratio2x1,
    Ratio3x1,
    Ratio4x5,
    Ratio3x4,
    Ratio2x3,
    Ratio3x5,
    Ratio9x16,
    Ratio1x2,
    Ratio1x3,
}

impl ImageSize {
    pub fn class(&self) -> &'static str {
        use ImageSize::*;
        match self {
            Pixels16x16 => "is-16x16",
            Pixels24x24 => "is-24x24",
            Pixels32x32 => "is-32x32",
            Pixels48x48 => "is-48x48",
            Pixels64x64 => "is-64x64",
            Pixels96x96 => "is-96x96",
            Pixels128x128 => "is-128x128",

            RatioSquare => "is-square",
            Ratio1x1 => "is-1by1",
            Ratio5x4 => "is-5by4",
            Ratio4x3 => "is-4by3",
            Ratio3x2 => "is-3by2",
            Ratio5x3 => "is-5by3",
            Ratio16x9 => "is-16by9",
            Ratio2x1 => "is-2by1",
            Ratio3x1 => "is-3by1",
            Ratio4x5 => "is-4by5",
            Ratio3x4 => "is-3by4",
            Ratio2x3 => "is-2by3",
            Ratio3x5 => "is-3by5",
            Ratio9x16 => "is-9by16",
            Ratio1x2 => "is-1by2",
            Ratio1x3 => "is-1by3",
        }
    }
}

impl PureComponent for PureImage {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        class.push("image");

        if let Some(size) = self.size {
            class.push(size.class());
        }

        if self.rounded {
            class.push("is-rounded");
        }

        if self.fullwidth {
            class.push("is-fullwidth")
        }

        html! {
            <figure class={class}>
                <img src={self.src.clone()} />
            </figure>
        }
    }
}
