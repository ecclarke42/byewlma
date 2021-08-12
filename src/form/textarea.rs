use std::borrow::Cow;

use derive_more::From;
use yew::{
    html::ChildrenRenderer,
    virtual_dom::{VChild, VNode},
};

use crate::components::{icon::IconProps, prelude::*, Icon};

// TODO: implement cursor for really big textareas?

pub type TextArea = Pure<PureTextArea>;

#[derive(Debug, Default, Clone, PartialEq, Properties)]
pub struct PureTextArea {
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,

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
}

// TODO: more input style attrs (placeholder, etc)

impl PureComponent for PureTextArea {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        unsafe {
            class.unchecked_push("textarea");
            if let Some(color) = &self.color {
                class.unchecked_push(color.is_class());
            }
            if let Some(size) = &self.size {
                class.unchecked_push(size.class());
            }
            if self.force_hover {
                class.unchecked_push("is-hovered");
            }
            if self.force_focus {
                class.unchecked_push("is-focused");
            }
            if self.is_loading {
                class.unchecked_push("is-loading");
            }
            if self.fix_size {
                class.unchecked_push("has-fixed-size");
            }
        }

        html! {
            <textarea
                class={class}
                rows={self.rows.map(|rows| rows.to_string())}
                placeholder={self.placeholder.clone()}

                disabled={self.is_disabled}
                readonly={self.is_readonly}
            />
        }
    }
}
