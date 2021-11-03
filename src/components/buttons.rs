// Validate all contents are buttons
// Are-Sizes
//https://bulma.io/documentation/elements/button/#sizes

use crate::innerlude::*;

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct ButtonsProps {
    #[prop_or_default]
    pub id: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub children: ChildrenWithProps<crate::components::Button>,

    /// Attach the buttons with "has-addons"
    #[prop_or_default]
    pub attached: bool,

    #[prop_or_default]
    pub alignment: ButtonsAlignment,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonsAlignment {
    Left,
    Center,
    Right,
}

impl Default for ButtonsAlignment {
    fn default() -> Self {
        Self::Left
    }
}

/// Bulma [List of Buttons](https://bulma.io/documentation/elements/button/#list-of-buttons)
#[function_component(Buttons)]
pub fn buttons(props: &ButtonsProps) -> Html {
    let mut class = props.class.clone();
    class.push("buttons");

    if props.attached {
        class.push("has-addons");
    }

    match props.alignment {
        ButtonsAlignment::Left => {}
        ButtonsAlignment::Center => {
            class.push("is-centered");
        }
        ButtonsAlignment::Right => {
            class.push("is-right");
        }
    }

    // TODO: is-selected

    html! {
        <div id={props.id.clone()} class={class} style={props.style.clone()}>
            {for props.children.iter()}
        </div>
    }
}
