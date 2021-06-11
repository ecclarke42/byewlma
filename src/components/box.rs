use crate::components::prelude::*;

/// Bulma [Box](https://bulma.io/documentation/elements/box/) Element
pub type Box = Pure<PureBox>;

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct PureBox {
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub(crate) children: Children,
}

impl PureComponent for PureBox {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        class.push("box");
        html! {
            <div id={self.id.clone()} class={class} style={self.style.clone()}>
                {for self.children.iter()}
            </div>
        }
    }
}
