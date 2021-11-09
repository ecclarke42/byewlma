use crate::{innerlude::*, SemanticColor, Size};

// TODO: implement cursor for really big textareas?

#[derive(Debug, Default, Clone, PartialEq, Properties)]
pub struct TextAreaProps {
    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

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
    pub placeholder: Option<AttrValue>,

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
    let id = props.id.clone();
    let style = props.style.clone();
    let value = props.value.clone();
    let rows = props.rows.map(|rows| AttrValue::Owned(rows.to_string()));
    let placeholder = props.placeholder.clone();

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

    let oninput = props.on_input.as_ref().map(|cb| {
        cb.reform(|evt: InputEvent| {
            evt.target_unchecked_into::<web_sys::HtmlTextAreaElement>()
                .value()
        })
    });

    let onchange = props.on_change.as_ref().map(|cb| {
        cb.reform(|evt: Event| {
            evt.target_unchecked_into::<web_sys::HtmlTextAreaElement>()
                .value()
        })
    });

    html! {
        <textarea {id} {class} {style} {value} {oninput} {onchange} {rows} {placeholder}
            disabled={props.is_disabled}
            readonly={props.is_readonly}
        />
    }
}
