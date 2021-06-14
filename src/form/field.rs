use crate::components::prelude::*;

/// Bulma [Field](https://bulma.io/documentation/form/general/) Element
pub type Field = Pure<PureField>;

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct PureField {
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub label: Option<String>,

    #[prop_or_default]
    pub help: Option<String>,
}

impl PureComponent for PureField {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        class.push("field");

        let label = if let Some(label) = &self.label {
            html! { <label class="label">{label.clone()}</label> }
        } else {
            html! {}
        };

        let help = if let Some(help) = &self.help {
            html! { <p class="help">{help.clone()}</p> }
        } else {
            html! {}
        };

        html! {
            <div id={self.id.clone()} class={class} style={self.style.clone()}>
                { label }
                { self.children.clone() }
                { help }
            </div>
        }
    }
}
