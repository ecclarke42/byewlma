use crate::innerlude::*;

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct BlockProps {
    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,
}

/// Bulma [Block](https://bulma.io/documentation/elements/block/) Element
#[function_component(Block)]
pub fn block(props: &BlockProps) -> Html {
    let id = props.id.clone();
    let style = props.style.clone();

    let mut class = props.class.clone();
    unsafe {
        class.unchecked_push("block");
    }
    html! {
        <div {id} {class} {style}>
            { for props.children.iter() }
        </div>
    }
}
