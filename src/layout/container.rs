use crate::innerlude::*;

/// Bulma [Container](https://bulma.io/documentation/layout/container/) Layout Element
pub type Container = Pure<PureContainer>;

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct PureContainer {
    #[prop_or_default]
    pub id: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub children: Children,

    /// Control max-width
    ///
    /// Default behavior: full width until $desktop breakpoint (max 960px),
    /// wider after $widescreen (1152px) and $fullhd (1344px);
    #[prop_or_default]
    pub width: Option<ContainerWidth>,
}

#[derive(Debug, Clone, Copy, PartialEq, BulmaClass)]
pub enum ContainerWidth {
    /// Full width until $widescreen breakpoint
    #[bulma_class = "is-widescreen"]
    FullUntilWidescreen,

    /// Full width until $fullhd breakpoint
    #[bulma_class = "is-fullhd"]
    FullUntilFullHd,

    /// Max width at $desktop breakpoint and wider
    #[bulma_class = "is-max-desktop"]
    MaxDesktop,

    /// Maxwidth at $widescreen breakpoint and wider
    #[bulma_class = "is-max-widescreen"]
    MaxWidescreen,

    /// Always full width, but with a 32px margin on each side
    #[bulma_class = "is-fluid"]
    Fluid,
}

impl PureComponent for PureContainer {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        unsafe {
            class.unchecked_push("container");
        }

        if let Some(width) = &self.width {
            class.add(width);
        }

        html! {
            <div id={self.id.clone()} class={class} style={self.style.clone()}>
                {for self.children.iter()}
            </div>
        }
    }
}
