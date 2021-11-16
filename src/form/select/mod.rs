use std::rc::Rc;

use yew::{prelude::*, virtual_dom::AttrValue};

#[derive(Debug, PartialEq, Properties)]
pub struct SelectOneProps {
    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    // selected: T,
    pub selected_index: usize,
    pub options: Rc<[String]>,

    #[prop_or_default]
    pub on_select: Option<Callback<usize>>,
}

#[function_component(SelectOne)]
pub fn select_one(props: &SelectOneProps) -> Html {
    let id = props.id.clone();
    let mut class = props.class.clone();
    let style = props.style.clone();

    unsafe {
        class.unchecked_push("select");
    }

    let options = props.options.iter().enumerate().map(|(i, opt)| {
        let onclick = props.on_select.as_ref().map(|cb| cb.reform(move |_| i));
        html! {
            <option value={opt.clone()} {onclick}>
                {opt.clone()}
            </option>
        }
    });

    html! {
        <div {id} {class} {style}>
            <select>
                { for options }
            </select>
        </div>
    }
}

// #[derive(Debug, PartialEq, Properties)]
// pub struct SelectEnumOptionProps {}

// #[function_component(SelectEnumOption)]
// pub fn select_enum_option(props: &SelectEnumOptionProps) -> Html {
//     html! {
//         <option>
//             {}
//         </option>
//     }
// }
