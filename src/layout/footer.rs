use crate::innerlude::*;

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct FooterProps {
    #[prop_or_default]
    pub id: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub children: Children,
}

/// Bulma [Footer](https://bulma.io/documentation/layout/footer/) Layout Element
#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    let mut class = props.class.clone();
    unsafe {
        class.unchecked_push("footer");
    }
    html! {
        <footer id={props.id.clone()} class={class} style={props.style.clone()}>
            {for props.children.iter()}
        </footer>
    }
}
