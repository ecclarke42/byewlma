use crate::innerlude::*;

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct BoxProps {
    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,
}

/// Bulma [Box](https://bulma.io/documentation/elements/box/) Element
#[function_component(Box)]
pub fn r#box(props: &BoxProps) -> Html {
    let id = props.id.clone();
    let style = props.style.clone();

    let mut class = props.class.clone();
    unsafe {
        class.unchecked_push("box");
    }
    html! {
        <div {id} {class} {style}>
            { for props.children.iter() }
        </div>
    }
}
