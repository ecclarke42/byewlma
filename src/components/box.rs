use crate::innerlude::*;

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct BoxProps {
    #[prop_or_default]
    pub id: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub children: Children,
}

/// Bulma [Box](https://bulma.io/documentation/elements/box/) Element
#[function_component(Box)]
pub fn r#box(props: &BoxProps) -> Html {
    let mut class = props.class.clone();
    unsafe {
        class.unchecked_push("box");
    }
    html! {
        <div id={props.id.clone()} class={class} style={props.style.clone()}>
            { for props.children.iter() }
        </div>
    }
}
