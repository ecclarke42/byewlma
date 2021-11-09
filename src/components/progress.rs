use crate::{innerlude::*, SemanticColor, Size};

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct ProgressProps {
    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

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
    let id = props.id.clone();
    let style = props.style.clone();

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
            <progress {id} {class} {style} />
        },

        ProgressState::Value { value, max } => html! {
            <progress {id} {class} {style}
                value={value.to_string()}
                max={max.to_string()}
            >
                // TODO: progress as formatted percent?
            </progress>
        },
    }
}
