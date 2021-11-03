use crate::{innerlude::*, SemanticColor, Size};

// TODO: implement cursor for really big textareas?

#[derive(Debug, Default, Clone, PartialEq, Properties)]
pub struct TextAreaProps {
    #[prop_or_default]
    pub id: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,

    // TODO: tab_index
    // TODO: name
    #[prop_or_default]
    pub name: Option<Cow<'static, str>>,
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub on_input: Option<Callback<String>>,
    #[prop_or_default]
    pub on_change: Option<Callback<String>>,

    #[prop_or_default]
    pub rows: Option<u32>,

    #[prop_or_default]
    pub fix_size: bool,

    #[prop_or_default]
    pub color: Option<SemanticColor>,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub placeholder: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub force_hover: bool,
    #[prop_or_default]
    pub force_focus: bool,
    #[prop_or_default]
    pub is_loading: bool, // TODO: control container
    #[prop_or_default]
    pub is_disabled: bool,
    #[prop_or_default]
    pub is_readonly: bool,
    #[prop_or_default]
    pub is_static: bool,
    // TODO On change/input
}

// TODO: more input style attrs (placeholder, etc)

#[function_component(TextArea)]
pub fn text_area(props: &TextAreaProps) -> Html {
    let mut class = props.class.clone();
    unsafe {
        class.unchecked_push("textarea");
        if let Some(color) = &props.color {
            class.add(color);
        }
        if let Some(size) = &props.size {
            class.add(size);
        }
        if props.force_hover {
            class.unchecked_push("is-hovered");
        }
        if props.force_focus {
            class.unchecked_push("is-focused");
        }
        if props.is_loading {
            class.unchecked_push("is-loading");
        }
        if props.fix_size {
            class.unchecked_push("has-fixed-size");
        }
    }

    let on_input = props.on_input.as_ref().map(|cb| {
        cb.reform(|evt: InputEvent| {
            evt.target_unchecked_into::<web_sys::HtmlTextAreaElement>()
                .value()
        })
    });

    let on_change = props.on_change.as_ref().map(|cb| {
        cb.reform(|evt: Event| {
            evt.target_unchecked_into::<web_sys::HtmlTextAreaElement>()
                .value()
        })
    });

    html! {
        <textarea
            id={props.id.clone()}
            class={class}
            style={props.style.clone()}

            value={props.value.clone()}
            oninput={on_input}
            onchange={on_change}

            rows={props.rows.map(|rows| rows.to_string())}
            placeholder={props.placeholder.clone()}

            disabled={props.is_disabled}
            readonly={props.is_readonly}

        />
    }
}
