use crate::components::prelude::*;

/// Bulma [Container](https://bulma.io/documentation/layout/container/) Layout Element
pub type Container = Pure<PureContainer>;

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct PureContainer {
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub children: Children,

    /// Control max-width
    ///
    /// Default behavior: full width until $desktop breakpoint (max 960px),
    /// wider after $widescreen (1152px) and $fullhd (1344px);
    #[prop_or_default]
    pub width: Option<ContainerWidth>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContainerWidth {
    /// Full width until $widescreen breakpoint
    FullUntilWidescreen,

    /// Full width until $fullhd breakpoint
    FullUntilFullHd,

    /// Max width at $desktop breakpoint and wider
    MaxDesktop,

    /// Maxwidth at $widescreen breakpoint and wider
    MaxWidescreen,

    /// Always full width, but with a 32px margin on each side
    Fluid,
}

impl PureComponent for PureContainer {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        class.push("container");

        match &self.width {
            None => {}
            Some(ContainerWidth::FullUntilWidescreen) => {
                class.push("is-widescreen");
            }
            Some(ContainerWidth::FullUntilFullHd) => {
                class.push("is-fullhd");
            }
            Some(ContainerWidth::MaxDesktop) => {
                class.push("is-max-desktop");
            }
            Some(ContainerWidth::MaxWidescreen) => {
                class.push("is-max-widescreen");
            }
            Some(ContainerWidth::Fluid) => {
                class.push("is-fluid");
            }
        }

        html! {
            <div id={self.id.clone()} class={class} style={self.style.clone()}>
                {for self.children.iter()}
            </div>
        }
    }
}
