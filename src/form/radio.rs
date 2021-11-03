use crate::innerlude::*;

// #[derive(Debug, Default, Clone, PartialEq, Properties)]
// pub struct PureRadioGroup<T> {
//     #[prop_or_default]
//     pub id: Option<Cow<'static, str>>,

//     #[prop_or_default]
//     pub class: Classes,

//     #[prop_or_default]
//     pub style: Option<Cow<'static, str>>,

//     #[prop_or_default]
//     pub name: Option<Cow<'static, str>>,

//     // #[prop_or_default]
//     // pub children: ChildrenWithProps<RadioButton<T>>,
//     #[prop_or_default]
//     pub children: Children,

//     #[prop_or_default]
//     pub on_change: Option<Callback<T>>,
// }

// impl<T> PureComponent for PureRadioGroup<T> {
//     fn render(&self) -> Html {
//         html! {
//             <>
//                 {self.children.clone()}
//             </>
//         }
//     }
// }

#[derive(Debug, Default, Clone, PartialEq, Properties)]
pub struct RadioButtonProps<T: Clone + PartialEq> {
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

    pub group: Cow<'static, str>,
    pub value: T,

    #[prop_or_default]
    pub is_checked: bool,

    #[prop_or_default]
    pub is_disabled: bool,

    #[prop_or_default]
    pub on_changed: Option<Callback<T>>,
}

// TODO: name/value for form compatibility?

#[function_component(RadioButton)]
pub fn radio_button<T: 'static + Clone + PartialEq>(props: &RadioButtonProps<T>) -> Html {
    let value = props.value.clone();
    let callback = props
        .on_changed
        .clone()
        .map(|cb| cb.reform(move |_| value.clone()));

    let mut class = props.class.clone();
    unsafe {
        class.unchecked_push("radio");
    }

    html! {
        <label id={props.label_id.clone()} class={class} style={props.style.clone()}>
            <input id={props.input_id.clone()} type="radio" name={props.group.clone()} checked={props.is_checked} disabled={props.is_disabled} onclick={callback} />
            { props.children.clone() }
        </label>
    }
}
