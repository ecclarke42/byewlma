use crate::innerlude::*;

/// Bulma [Image](https://bulma.io/documentation/elements/image/) Element
pub type Image = Pure<PureImage>;

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct PureImage {
    #[prop_or_default]
    pub id: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,

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
#[derive(Debug, Clone, Copy, PartialEq, BulmaClass)]
pub enum ImageSize {
    #[bulma_class = "is-16x16"]
    Pixels16x16,
    #[bulma_class = "is-24x24"]
    Pixels24x24,
    #[bulma_class = "is-32x32"]
    Pixels32x32,
    #[bulma_class = "is-48x48"]
    Pixels48x48,
    #[bulma_class = "is-64x64"]
    Pixels64x64,
    #[bulma_class = "is-96x96"]
    Pixels96x96,
    #[bulma_class = "is-128x128"]
    Pixels128x128,

    #[bulma_class = "is-square"]
    RatioSquare,
    #[bulma_class = "is-1by1"]
    Ratio1x1,
    #[bulma_class = "is-5by4"]
    Ratio5x4,
    #[bulma_class = "is-4by3"]
    Ratio4x3,
    #[bulma_class = "is-3by2"]
    Ratio3x2,
    #[bulma_class = "is-5by3"]
    Ratio5x3,
    #[bulma_class = "is-16by9"]
    Ratio16x9,
    #[bulma_class = "is-2by1"]
    Ratio2x1,
    #[bulma_class = "is-3by1"]
    Ratio3x1,
    #[bulma_class = "is-4by5"]
    Ratio4x5,
    #[bulma_class = "is-3by4"]
    Ratio3x4,
    #[bulma_class = "is-2by3"]
    Ratio2x3,
    #[bulma_class = "is-3by5"]
    Ratio3x5,
    #[bulma_class = "is-9by16"]
    Ratio9x16,
    #[bulma_class = "is-1by2"]
    Ratio1x2,
    #[bulma_class = "is-1by3"]
    Ratio1x3,
}

impl PureComponent for PureImage {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        unsafe {
            class.unchecked_push("image");

            if let Some(size) = &self.size {
                class.add(size);
            }

            if self.rounded {
                class.unchecked_push("is-rounded");
            }

            if self.fullwidth {
                class.unchecked_push("is-fullwidth")
            }
        }

        html! {
            <figure id={self.id.clone()} class={class} style={self.style.clone()}>
                <img src={self.src.clone()} />
            </figure>
        }
    }
}
