// TODO: buttons container

// Validate all contents are buttons
// Are-Sizes
//https://bulma.io/documentation/elements/button/#sizes

use crate::components::prelude::*;

/// Bulma [List of Buttons](https://bulma.io/documentation/elements/button/#list-of-buttons)
pub type Buttons = Pure<PureButtons>;

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct PureButtons {
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub(crate) children: ChildrenWithProps<crate::components::Button>,

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

impl PureComponent for PureButtons {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        class.push("buttons");

        if self.attached {
            class.push("has-addons");
        }

        match self.alignment {
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
            <div id={self.id.clone()} class={class} style={self.style.clone()}>
                {for self.children.iter()}
            </div>
        }
    }
}
