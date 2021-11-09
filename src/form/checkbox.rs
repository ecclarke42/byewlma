use crate::innerlude::*;

#[derive(Debug, Default, Clone, PartialEq, Properties)]
pub struct CheckboxProps {
    /// Id for the wrapping label
    #[prop_or_default]
    pub label_id: Option<AttrValue>,

    /// Id for the input element itself
    #[prop_or_default]
    pub input_id: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub is_checked: bool,

    #[prop_or_default]
    pub is_disabled: bool,

    #[prop_or_default]
    pub on_toggled: Option<Callback<()>>,
}

// TODO: name/value for form compatibility?

#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    let label_id = props.label_id.clone();
    let label_style = props.style.clone();
    let input_id = props.input_id.clone();

    let onclick = props.on_toggled.as_ref().map(|cb| cb.reform(|_| ()));

    let mut class = props.class.clone();
    unsafe {
        class.unchecked_push("checkbox");
    }

    html! {
        <label id={label_id} {class} style={label_style} >
            <input id={input_id} type="checkbox" checked={props.is_checked} disabled={props.is_disabled} {onclick} />
            { props.children.clone() }
        </label>
    }
}

// TODO: CheckboxGroup?
