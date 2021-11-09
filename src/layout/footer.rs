use crate::innerlude::*;

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct FooterProps {
    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,
}

/// Bulma [Footer](https://bulma.io/documentation/layout/footer/) Layout Element
#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    let id = props.id.clone();
    let style = props.style.clone();

    let mut class = props.class.clone();
    unsafe {
        class.unchecked_push("footer");
    }
    html! {
        <footer {id} {class} {style}>
            {for props.children.iter()}
        </footer>
    }
}
