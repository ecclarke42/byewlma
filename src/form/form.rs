use std::borrow::Cow;

use crate::components::prelude::*;

// pub type HorizontalForm = Pure<PureHorizontalForm>;

// #[derive(Debug, Default, Clone, PartialEq, Properties)]
// pub struct PureHorizontalForm {
//     #[prop_or_default]
//     pub id: Option<String>

//     #[prop_or_default]
//     pub class: Classes,

//     #[prop_or_default]
//     pub style: Option<Cow<'static, str>>,

//     #[prop_or_default]
//     pub children: ChildrenWithProps<HorizontalFormRow>,
// }

// impl PureComponent for PureHorizontalForm {
//     fn render(&self) -> Html {

//     }
// }

pub type FormRow = Pure<PureFormRow>;

#[derive(Debug, Default, Clone, PartialEq, Properties)]
pub struct PureFormRow {
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub label: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub label_size: Option<Size>,
}

impl PureComponent for PureFormRow {
    fn render(&self) -> Html {
        let label = if let Some(label) = &self.label {
            let mut label_class = classes!("field-label");
            if let Some(size) = self.label_size {
                unsafe { label_class.unchecked_push(size.class()) }
            }
            html! {
                <div class={label_class}>
                    <label class="label">{label}</label>
                </div>
            }
        } else {
            html! {}
        };
        html! {
            <div id={self.id.clone()} class="field is-horizontal" style={self.style.clone()}>
                {label}
                <div class="field-body">
                    { for self.children.clone() }
                </div>
            </div>
        }
    }
}

// TODO: disabled fieldset https://bulma.io/documentation/form/general/#disabled-form
