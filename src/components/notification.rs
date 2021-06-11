use crate::{components::prelude::*, SemanticOrLightColor};

/// Bulma [Notification](https://bulma.io/documentation/elements/notification/) Element
pub type Notification = Pure<PureNotification>;

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct PureNotification {
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub(crate) children: Children,

    #[prop_or_default]
    pub color: Option<SemanticOrLightColor>,

    #[prop_or_default]
    pub light: bool,

    /// To display a delete button in the notification, pass a callback to
    /// trigger on click
    #[prop_or_default]
    pub on_close: Option<Callback<()>>,
}

impl PureComponent for PureNotification {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        class.push("notification");

        match self.color {
            None => {}
            Some(SemanticOrLightColor::Normal(color)) => {
                class.push(color.is_class());
            }
            Some(SemanticOrLightColor::Light(color)) => {
                class.push(classes!(color.is_class(), "is-light"))
            }
        }

        let close_button = if let Some(callback) = &self.on_close {
            html! { <button class="delete" onclick={callback.reform(|_|())}/> }
        } else {
            html! {}
        };

        html! {
            <div id={self.id.clone()} class={class} style={self.style.clone()}>
                { close_button }
                { for self.children.iter() }
            </div>
        }
    }
}
