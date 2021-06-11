use crate::components::prelude::*;

/// Bulma [Block](https://bulma.io/documentation/elements/block/) Element
pub type Block = Pure<PureBlock>;

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct PureBlock {
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub(crate) children: Children,
}

impl PureComponent for PureBlock {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        class.push("block");
        html! {
            <div id={self.id.clone()} class={class} style={self.style.clone()}>
                {for self.children.iter()}
            </div>
        }
    }
}
