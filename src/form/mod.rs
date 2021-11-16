use crate::{innerlude::*, Size};

pub mod checkbox;
pub mod control;
pub mod field;
pub mod file;
pub mod input;
pub mod radio;
pub mod select;
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
use yew::virtual_dom::VNode;

// pub type HorizontalForm = Pure<PureHorizontalForm>;

// #[derive(Debug, Default, Clone, PartialEq, Properties)]
// pub struct PureHorizontalForm {
//     #[prop_or_default]
//     pub id: Option<AttrValue>,

//     #[prop_or_default]
//     pub class: Classes,

//     #[prop_or_default]
//     pub style: Option<AttrValue>,

//     #[prop_or_default]
//     pub children: ChildrenWithProps<HorizontalFormRow>,
// }

// impl PureComponent for PureHorizontalForm {
//     fn render(&self) -> Html {

//     }
// }

#[derive(Debug, Default, Clone, PartialEq, Properties)]
pub struct FormRowProps {
    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,

    // TODO: split label to separate component?
    #[prop_or_default]
    pub label: Option<FormLabel>,

    #[prop_or_default]
    pub label_size: Option<Size>,
}

#[function_component(FormRow)]
pub fn form_row(props: &FormRowProps) -> Html {
    let id = props.id.clone();
    let style = props.style.clone();

    let label = if let Some(label) = &props.label {
        let mut label_class = classes!("field-label");
        if let Some(size) = props.label_size {
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

    let mut class = props.class.clone();
    unsafe {
        class.unchecked_push("field");
        class.unchecked_push("is-horizontal");
    }

    html! {
        <div {id} {class} {style}>
            {label}
            <div class="field-body">
                { for props.children.iter() }
            </div>
        </div>
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

impl From<FormLabel> for Html {
    fn from(label: FormLabel) -> Self {
        match label {
            FormLabel::Text(text) => text.into(),
            FormLabel::Node(node) => node,
        }
    }
}
