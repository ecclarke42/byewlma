use crate::components::prelude::*;

/// Bulma [Progress](https://bulma.io/documentation/elements/block/) Element
pub type Progress = Pure<PureProgress>;

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct PureProgress {
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<String>,

    pub state: ProgressState,

    #[prop_or_default]
    pub size: Option<Size>,
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

impl PureComponent for PureProgress {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        class.push("progress");

        if let Some(size) = &self.size {
            class.push(size.class())
        }

        match self.state {
            ProgressState::Indeterminate => html! {
                <progress
                    id={self.id.clone()}
                    class={class}
                    style={self.style.clone()}
                />
            },

            ProgressState::Value { value, max } => html! {
                <progress
                    id={self.id.clone()}
                    class={class}
                    style={self.style.clone()}
                    value={value.to_string()}
                    max={max.to_string()}
                >
                    // TODO: progress as formatted percent?
                </progress>
            },
        }
    }
}