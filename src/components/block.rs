use crate::innerlude::*;

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct BlockProps {
    #[prop_or_default]
    pub id: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub children: Children,
}

/// Bulma [Block](https://bulma.io/documentation/elements/block/) Element
#[function_component(Block)]
pub fn block(props: &BlockProps) -> Html {
    let mut class = props.class.clone();
    unsafe {
        class.unchecked_push("block");
    }
    html! {
        <div id={props.id.clone()} class={class} style={props.style.clone()}>
            { for props.children.iter() }
        </div>
    }
}
