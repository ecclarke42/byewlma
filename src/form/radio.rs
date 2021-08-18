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

pub type RadioButton<T> = Pure<PureRadioButton<T>>;

#[derive(Debug, Default, Clone, PartialEq, Properties)]
pub struct PureRadioButton<T: Clone> {
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

impl<T: 'static + Clone + PartialEq> PureComponent for PureRadioButton<T> {
    fn render(&self) -> Html {
        let value = self.value.clone();
        let callback = self
            .on_changed
            .clone()
            .map(|cb| cb.reform(move |_| value.clone()));

        let mut class = self.class.clone();
        unsafe {
            class.unchecked_push("radio");
        }

        html! {
            <label id={self.label_id.clone()} class={class} style={self.style.clone()}>
                <input id={self.input_id.clone()} type="radio" name={self.group.clone()} checked={self.is_checked} disabled={self.is_disabled} onclick={callback} />
                { self.children.clone() }
            </label>
        }
    }
}
