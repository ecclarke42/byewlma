use crate::components::prelude::*;

pure_props! {
    /// Bulma [Content](https://bulma.io/documentation/elements/content/) Element
    pub struct Content {
        #[prop_or_default]
        pub size: Option<Size>,
    }
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
