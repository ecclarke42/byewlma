use crate::{innerlude::*, Size};

pub mod checkbox;
pub mod control;
pub mod field;
pub mod input;
pub mod radio;
pub mod textarea;
pub mod controls {
    // pub mod password_control;
}

pub use checkbox::Checkbox;
pub use control::Control;
// pub use controls::password_control::PasswordControl;
pub use field::Field;
pub use input::{
    ColorInput, DateInput, DateTimeInput, EmailInput, FloatInput, IntegerInput, MonthInput,
    PasswordInput, PhoneInput, PositiveIntegerInput, TextInput, TimeInput, UrlInput, WeekInput,
};
pub use radio::RadioButton;
pub use textarea::TextArea;
use yew::{html::IntoPropValue, virtual_dom::VNode};

// pub type HorizontalForm = Pure<PureHorizontalForm>;

// #[derive(Debug, Default, Clone, PartialEq, Properties)]
// pub struct PureHorizontalForm {
//     #[prop_or_default]
//     pub id: Option<Cow<'static, str>>,

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
    pub id: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub children: Children,

    // TODO: split label to separate component?
    #[prop_or_default]
    // pub label: Option<Cow<'static, str>>,
    pub label: Option<FormLabel>,

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
                    <label class="label">{label.clone()}</label>
                </div>
            }
        } else {
            html! {}
        };

        let mut class = self.class.clone();
        unsafe {
            class.unchecked_push("field");
            class.unchecked_push("is-horizontal");
        }

        html! {
            <div id={self.id.clone()} class={class} style={self.style.clone()}>
                {label}
                <div class="field-body">
                    { for self.children.clone() }
                </div>
            </div>
        }
    }
}

// TODO: disabled fieldset https://bulma.io/documentation/form/general/#disabled-form

#[derive(Debug, Clone, PartialEq)]
pub enum FormLabel {
    Text(Cow<'static, str>),
    Node(VNode),
}

// TODO: fix

// impl IntoPropValue<Option<FormLabel>> for &'static str {
//     fn into_prop_value(self) -> Option<FormLabel> {
//         Some(FormLabel::Text(self.into()))
//     }
// }

// impl IntoPropValue<Option<FormLabel>> for VNode {
//     fn into_prop_value(self) -> Option<FormLabel> {
//         Some(FormLabel::Node(self))
//     }
// }

impl From<Cow<'static, str>> for FormLabel {
    fn from(s: Cow<'static, str>) -> Self {
        FormLabel::Text(s)
    }
}

impl From<String> for FormLabel {
    fn from(s: String) -> Self {
        FormLabel::Text(s.into())
    }
}

impl From<&'static str> for FormLabel {
    fn from(s: &'static str) -> Self {
        FormLabel::Text(s.into())
    }
}

impl From<VNode> for FormLabel {
    fn from(node: VNode) -> Self {
        FormLabel::Node(node)
    }
}

impl Into<Html> for FormLabel {
    fn into(self) -> Html {
        match self {
            FormLabel::Text(text) => text.into(),
            FormLabel::Node(node) => node,
        }
    }
}
