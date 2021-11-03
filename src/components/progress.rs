use crate::{innerlude::*, SemanticColor, Size};

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct ProgressProps {
    #[prop_or_default]
    pub id: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,

    pub state: ProgressState,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub color: Option<SemanticColor>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProgressState {
    Indeterminate,
    Value { value: usize, max: usize },
}

impl Default for ProgressState {
    fn default() -> Self {
        Self::Indeterminate
    }
}

/// Bulma [Progress](https://bulma.io/documentation/elements/block/) Element
#[function_component(Progress)]
pub fn progress(props: &ProgressProps) -> Html {
    let mut class = props.class.clone();
    unsafe {
        class.unchecked_push("progress");
    }

    if let Some(size) = &props.size {
        class.add(size);
    }

    if let Some(color) = &props.color {
        class.add(color);
    }

    match props.state {
        ProgressState::Indeterminate => html! {
            <progress
                id={props.id.clone()}
                class={class}
                style={props.style.clone()}
            />
        },

        ProgressState::Value { value, max } => html! {
            <progress
                id={props.id.clone()}
                class={class}
                style={props.style.clone()}
                value={value.to_string()}
                max={max.to_string()}
            >
                // TODO: progress as formatted percent?
            </progress>
        },
    }
}
