use crate::innerlude::*;

#[derive(Debug, Default, Clone, PartialEq, Properties)]
pub struct CheckboxProps {
    /// Id for the wrapping label
    #[prop_or_default]
    pub label_id: Option<Cow<'static, str>>,

    /// Id for the input element itself
    #[prop_or_default]
    pub input_id: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,

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
    let callback = props.on_toggled.clone().map(|cb| cb.reform(|_| ()));
    let mut class = props.class.clone();
    unsafe {
        class.unchecked_push("checkbox");
    }
    html! {
        <label id={props.label_id.clone()} class={class} style={props.style.clone()} >
            <input id={props.input_id.clone()} type="checkbox" checked={props.is_checked} disabled={props.is_disabled} onclick={callback} />
            { props.children.clone() }
        </label>
    }
}

// TODO: CheckboxGroup?
