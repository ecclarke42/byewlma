use crate::innerlude::*;

use gloo::file::File;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{Event, HtmlInputElement};
use yew::TargetCast;

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct FileInputProps {
    // #[prop_or_default]
    // pub id: Option<Cow<'static, str>>,
    #[prop_or_default]
    pub class: Classes,
    // #[prop_or_default]
    // pub style: Option<Cow<'static, str>>,
    // #[prop_or_default]
    // pub tab_index: Option<i16>,

    // #[prop_or_default]
    // pub name: Option<Cow<'static, str>>,

    // TODO: value type?
    // #[prop_or_default]
    // pub value: Option<#value_type>,

    // #[prop_or_default]
    // pub on_input: Option<Callback<>>,
    #[prop_or_default]
    pub label: Cow<'static, str>,
    #[prop_or_default]
    pub label_class: Classes,

    #[prop_or_default]
    pub on_change: Option<Callback<Vec<File>>>,

    #[prop_or_default]
    pub file_name: Option<Cow<'static, str>>,
    #[prop_or_default]
    pub file_name_side: FileNameSide,
    #[prop_or_default]
    pub is_fullwidth: bool,
    #[prop_or_default]
    pub is_boxed: bool,
    #[prop_or_default]
    pub multiple: bool,

    // Yew Input Props
    #[prop_or_default]
    pub color: Option<crate::SemanticColor>, // TODO: semantic or white/black/etc.
    #[prop_or_default]
    pub size: Option<crate::Size>,
    #[prop_or_default]
    pub alignment: Option<FileInputAlignment>,
    // #[prop_or_default]
    // pub is_loading: bool, // TODO: control container
    #[prop_or_default]
    pub is_disabled: bool,
    #[prop_or_default]
    pub is_readonly: bool,
    // #[prop_or_default]
    // pub is_static: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileNameSide {
    Left,
    Right,
}

impl Default for FileNameSide {
    fn default() -> Self {
        FileNameSide::Left
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileInputAlignment {
    Left,
    Center,
    Right,
}

impl Default for FileInputAlignment {
    fn default() -> Self {
        FileInputAlignment::Left
    }
}

#[function_component(FileInput)]
pub fn file_input(props: &FileInputProps) -> Html {
    let onchange = props.on_change.as_ref().map(|cb| {
        cb.reform(|evt: Event| {
            let target: HtmlInputElement = evt.target_unchecked_into();
            target
                .files()
                .map(|file_list| {
                    let length = file_list.length();
                    let mut files = Vec::with_capacity(length as usize);
                    for index in 0..length {
                        files.push(File::from(file_list.get(index).unwrap_throw()));
                    }
                    files
                })
                .unwrap_or_default()
        })
    });

    let mut class = props.class.clone();
    unsafe { class.unchecked_push("file") }

    let mut label_class = props.label_class.clone();
    unsafe { label_class.unchecked_push("file-cta") }

    let name = props
        .file_name
        .as_ref()
        .map(|name| {
            unsafe {
                class.unchecked_push("has-name");
            }
            html! {
                <span class="file-name">
                    {name}
                </span>
            }
        })
        .unwrap_or_default();

    match props.file_name_side {
        FileNameSide::Left => {}
        FileNameSide::Right => unsafe {
            class.unchecked_push("is-right");
        },
    }

    if props.is_fullwidth {
        unsafe { class.unchecked_push("is-fullwidth") }
    }

    if props.is_boxed {
        unsafe { class.unchecked_push("is-boxed") }
    }

    if let Some(color) = &props.color {
        unsafe { class.unchecked_push(color.class()) }
    }

    if let Some(size) = &props.size {
        unsafe { class.unchecked_push(size.class()) }
    }

    match &props.alignment {
        None | Some(FileInputAlignment::Left) => {}
        Some(FileInputAlignment::Center) => unsafe { class.unchecked_push("is-centered") },
        Some(FileInputAlignment::Right) => unsafe { class.unchecked_push("is-right") },
    }

    html! {
        <div {class}>
            <label class="file-label">
                <input
                    type="file"
                    class="file-input"

                    // id={props.id.clone()}
                    // style={props.style.clone()}
                    // tabindex={props.tab_index.map(|x| x.to_string())}

                    // name={props.name.clone()}
                    // value={value}
                    // oninput={on_input}
                    {onchange}

                    disabled={props.is_disabled}
                    readonly={props.is_readonly}

                    multiple={props.multiple}
                />
                <span class={label_class}>
                    <span class="file-icon">
                        <i class="fas fa-upload"></i>
                    </span>
                    <span class="file-label">
                        {props.label.clone()}
                    </span>
                </span>
                {name}
            </label>
        </div>
    }
}
