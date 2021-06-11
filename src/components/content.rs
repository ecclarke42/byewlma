use crate::components::prelude::*;

/// Bulma [Content](https://bulma.io/documentation/elements/content/) Element
pub type Content = Pure<PureContent>;

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct PureContent {
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub(crate) children: Children,

    #[prop_or_default]
    pub size: Option<Size>,
}

impl PureComponent for PureContent {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        class.push("content");

        if let Some(size) = self.size {
            class.push(size);
        }

        html! {
            <div id={self.id.clone()} class={class} style={self.style.clone()}>
                {for self.children.iter()}
            </div>
        }
    }
}
