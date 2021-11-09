use crate::{innerlude::*, SemanticOrLightColor};

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct NotificationProps {
    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub color: Option<SemanticOrLightColor>,

    #[prop_or_default]
    pub light: bool,

    /// To display a delete button in the notification, pass a callback to
    /// trigger on click
    #[prop_or_default]
    pub on_close: Option<Callback<()>>,
}

/// Bulma [Notification](https://bulma.io/documentation/elements/notification/) Element
#[function_component(Notification)]
pub fn notification(props: &NotificationProps) -> Html {
    let id = props.id.clone();
    let style = props.style.clone();

    let mut class = props.class.clone();
    unsafe {
        class.unchecked_push("notification");
    }

    match &props.color {
        None => {}
        Some(SemanticOrLightColor::Normal(color)) => {
            class.add(color);
        }
        Some(SemanticOrLightColor::Light(color)) => {
            class.add(color);
            unsafe {
                class.unchecked_push("is-light");
            }
        }
    }

    let close_button = if let Some(callback) = &props.on_close {
        html! { <button class="delete" onclick={callback.reform(|_|())}/> }
    } else {
        html! {}
    };

    html! {
        <div {id} {class} {style}>
            { close_button }
            { for props.children.iter() }
        </div>
    }
}
