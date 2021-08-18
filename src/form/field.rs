use crate::{innerlude::*, SemanticColor, Size};

/// Bulma [Field](https://bulma.io/documentation/form/general/) Element
///
/// In Bulma, the field div often needs the inner control or input to have extra
/// classes
pub type Field = Pure<PureField>;

#[derive(Debug, Default, Clone, PartialEq, Properties)]
pub struct PureField {
    #[prop_or_default]
    pub id: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub children: Children, // ChildrenWithProps<super::Control>,

    #[prop_or_default]
    pub label: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub help: Option<Cow<'static, str>>,
    #[prop_or_default]
    pub help_color: Option<SemanticColor>,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub layout: Option<FieldLayout>,
}

// TODO: color for helpers

/// Default is Grouped
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldLayout {
    AddonsLeft,
    AddonsCenter,
    AddonsRight,

    GroupedLeft,
    GroupedCenter,
    GroupedRight,

    GroupedMultiline,
}
impl Default for FieldLayout {
    fn default() -> Self {
        FieldLayout::GroupedLeft
    }
}
impl FieldLayout {
    pub fn has_addons(&self) -> bool {
        use FieldLayout::*;
        matches!(self, AddonsLeft | AddonsCenter | AddonsRight)
    }
    pub fn is_multiline(&self) -> bool {
        use FieldLayout::*;
        matches!(self, GroupedMultiline)
    }
}

impl PureComponent for PureField {
    fn render(&self) -> Html {
        let mut field_class = self.class.clone();
        unsafe {
            field_class.unchecked_push("field");
        }
        let mut label_class = classes!("label");

        match &self.size {
            Some(Size::Normal) | None => {}
            Some(size) => {
                let size_class = size.class();
                unsafe {
                    label_class.unchecked_push(size_class);
                }
            }
        }

        if self.children.len() > 1 {
            use FieldLayout::*;
            unsafe {
                match &self.layout {
                    Some(AddonsCenter) => {
                        field_class.unchecked_push("has-addons");
                        field_class.unchecked_push("has-addons-centered");
                    }
                    Some(AddonsRight) => {
                        field_class.unchecked_push("has-addons");
                        field_class.unchecked_push("has-addons-right");
                    }
                    Some(GroupedMultiline) => {
                        field_class.unchecked_push("is-grouped");
                        field_class.unchecked_push("is-grouped-multiline");
                    }
                    Some(GroupedCenter) => {
                        field_class.unchecked_push("is-grouped");
                        field_class.unchecked_push("is-grouped-centered");
                    }
                    Some(GroupedRight) => {
                        field_class.unchecked_push("is-grouped");
                        field_class.unchecked_push("is-grouped-right");
                    }
                    Some(AddonsLeft) => field_class.unchecked_push("has-addons"),
                    Some(GroupedLeft) => field_class.unchecked_push("is-grouped"),
                    None => {}
                }
            }
        }

        let label = if let Some(label) = &self.label {
            html! { <label class={label_class}>{label.clone()}</label> }
        } else {
            html! {}
        };

        let help = if let Some(help) = &self.help {
            let mut class = Classes::new();
            unsafe {
                class.unchecked_push("help");
            }
            if let Some(color) = &self.help_color {
                class.add(color);
            }
            html! { <p class={class}>{help.clone()}</p> }
        } else {
            html! {}
        };

        html! {
            <div id={self.id.clone()} class={field_class} style={self.style.clone()}>
                { label }
                { for self.children.iter() }
                { help }
            </div>
        }
    }
}
