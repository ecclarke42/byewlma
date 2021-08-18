use crate::innerlude::*;

pub type Checkbox = Pure<PureCheckbox>;

#[derive(Debug, Default, Clone, PartialEq, Properties)]
pub struct PureCheckbox {
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

impl PureComponent for PureCheckbox {
    fn render(&self) -> Html {
        let callback = self.on_toggled.clone().map(|cb| cb.reform(|_| ()));
        let mut class = self.class.clone();
        unsafe {
            class.unchecked_push("checkbox");
        }
        html! {
            <label id={self.label_id.clone()} class={class} style={self.style.clone()} >
                <input id={self.input_id.clone()} type="checkbox" checked={self.is_checked} disabled={self.is_disabled} onclick={callback} />
                { self.children.clone() }
            </label>
        }
    }
}

// TODO: CheckboxGroup?
