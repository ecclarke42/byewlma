use crate::{helpers::Size, innerlude::*};

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct ContentProps {
    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub size: Option<Size>,
}

/// Bulma [Content](https://bulma.io/documentation/elements/content/) Element
#[function_component(Content)]
pub fn content(props: &ContentProps) -> Html {
    let id = props.id.clone();
    let style = props.style.clone();

    let mut class = props.class.clone();
    unsafe { class.unchecked_push("content") }

    if let Some(size) = &props.size {
        class.add(size);
    }

    html! {
        <div {id} {class} {style}>
            {for props.children.iter()}
        </div>
    }
}
