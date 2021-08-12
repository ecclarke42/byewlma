use std::borrow::Cow;

use derive_more::From;
use yew::{
    html::ChildrenRenderer,
    virtual_dom::{VChild, VNode},
};

use crate::components::{icon::IconProps, prelude::*, Icon};

/// Bulma [Field](https://bulma.io/documentation/form/general/) Element
///
/// In Bulma, the field div often needs the inner control or input to have extra
/// classes
pub type Field = Pure<PureField>;

#[derive(Debug, Default, Clone, PartialEq, Properties)]
pub struct PureField {
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub children: ChildrenRenderer<FieldChild>,

    #[prop_or_default]
    pub label: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub help: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub left_icon: Option<IconProps>,

    #[prop_or_default]
    pub right_icon: Option<IconProps>,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub layout: FieldLayout,
}

/// Default is Grouped
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldLayout {
    AddonsFullWidth { expand_index: usize },
    AddonsLeft,
    AddonsCenter,
    AddonsRight,

    GroupedFullWidth { expand_index: usize },
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
        matches!(
            self,
            AddonsFullWidth { .. } | AddonsLeft | AddonsCenter | AddonsRight
        )
    }
    pub fn is_expanded(&self) -> bool {
        use FieldLayout::*;
        matches!(self, AddonsFullWidth { .. } | GroupedFullWidth { .. })
    }
    pub fn is_multiline(&self) -> bool {
        use FieldLayout::*;
        matches!(self, GroupedMultiline)
    }
    pub fn expanded_index(&self) -> Option<usize> {
        use FieldLayout::*;
        if let AddonsFullWidth { expand_index } | GroupedFullWidth { expand_index } = self {
            Some(*expand_index)
        } else {
            None
        }
    }
}

impl PureComponent for PureField {
    fn render(&self) -> Html {
        let mut field_class = self.class.clone();
        unsafe {
            field_class.unchecked_push("field");
        }

        let mut label_class = classes!("label");
        let mut control_class = classes!("control");
        let mut child_class = classes!();

        // TODO: Icon size?
        match &self.size {
            Some(Size::Normal) | None => {}
            Some(size) => {
                let size_class = size.class();
                unsafe {
                    label_class.unchecked_push(size_class);
                    // control
                }
            }
        }

        if self.children.len() > 1 {
            use FieldLayout::*;
            unsafe {
                match &self.layout {
                    AddonsCenter => {
                        field_class.unchecked_push("has-addons");
                        field_class.unchecked_push("has-addons-centered");
                    }
                    AddonsRight => {
                        field_class.unchecked_push("has-addons");
                        field_class.unchecked_push("has-addons-right");
                    }
                    GroupedMultiline => {
                        field_class.unchecked_push("is-grouped");
                        field_class.unchecked_push("is-grouped-multiline");
                    }
                    GroupedCenter => {
                        field_class.unchecked_push("is-grouped");
                        field_class.unchecked_push("is-grouped-centered");
                    }
                    GroupedRight => {
                        field_class.unchecked_push("is-grouped");
                        field_class.unchecked_push("is-grouped-right");
                    }
                    AddonsLeft | AddonsFullWidth { .. } => field_class.unchecked_push("has-addons"),
                    GroupedLeft | GroupedFullWidth { .. } => {
                        field_class.unchecked_push("is-grouped")
                    }
                }
            }
        } else if self.layout.is_expanded() {
            unsafe { control_class.unchecked_push("is-expanded") }
        }

        let label = if let Some(label) = &self.label {
            html! { <label class={label_class}>{label.clone()}</label> }
        } else {
            html! {}
        };

        let help = if let Some(help) = &self.help {
            html! { <p class="help">{help.clone()}</p> }
        } else {
            html! {}
        };

        let left_icon = if let Some(icon_props) = &self.left_icon {
            let mut icon_props = icon_props.clone();
            unsafe {
                control_class.unchecked_push("has-icons-left");
                icon_props.class.unchecked_push("is-left");
            }
            // TODO: Icon props/size?

            html! { <Icon with icon_props />
                // <span class="icon is-small is-left"> // TODO: icon size? and color?
                //     <i class={classes!("fas", icon_kind.name())}/>
                // </span>
            }
        } else {
            html! {}
        };

        let right_icon = if let Some(icon_props) = &self.right_icon {
            let mut icon_props = icon_props.clone();
            unsafe {
                control_class.unchecked_push("has-icons-right");
                icon_props.class.unchecked_push("is-right");
            }
            html! { <Icon with icon_props /> }
        } else {
            html! {}
        };

        let expanded_index = self.layout.expanded_index();
        let children = self.children.iter().enumerate().map(move |(i, mut child)| {
            let is_expanded = expanded_index.map(|idx| idx == i).unwrap_or_default();

            match &mut child {
                FieldChild::ColorInput(VChild { props, .. }) => {
                    props.class.push(child_class.clone())
                }
                FieldChild::DateInput(VChild { props, .. }) => {
                    props.class.push(child_class.clone())
                }
                FieldChild::DateTimeInput(VChild { props, .. }) => {
                    props.class.push(child_class.clone())
                }
                FieldChild::EmailInput(VChild { props, .. }) => {
                    props.class.push(child_class.clone())
                }
                FieldChild::MonthInput(VChild { props, .. }) => {
                    props.class.push(child_class.clone())
                }
                FieldChild::IntegerInput(VChild { props, .. }) => {
                    props.class.push(child_class.clone())
                }
                FieldChild::PositiveIntegerInput(VChild { props, .. }) => {
                    props.class.push(child_class.clone())
                }
                FieldChild::FloatInputInput(VChild { props, .. }) => {
                    props.class.push(child_class.clone())
                }
                FieldChild::PhoneInput(VChild { props, .. }) => {
                    props.class.push(child_class.clone())
                }
                FieldChild::TextInput(VChild { props, .. }) => {
                    props.class.push(child_class.clone())
                }
                FieldChild::TimeInput(VChild { props, .. }) => {
                    props.class.push(child_class.clone())
                }
                FieldChild::UrlInput(VChild { props, .. }) => props.class.push(child_class.clone()),
                FieldChild::WeekInput(VChild { props, .. }) => {
                    props.class.push(child_class.clone())
                }

                // TODO: if select, add "select is-fullwidth"
                _ => {}
            }

            let mut control_class = control_class.clone();
            if is_expanded {
                unsafe { control_class.unchecked_push("is-expanded") }
            }

            html! {
                <div class={control_class}>
                    { child }
                </div>
            }
        });

        html! {
            <div id={self.id.clone()} class={field_class} style={self.style.clone()}>
                { label }
                { for children }
                { left_icon }
                { right_icon }
                { help }
            </div>
        }
    }
}

// TODO: just use super::input::Input somehow?
#[derive(Clone, From)]
pub enum FieldChild {
    ColorInput(VChild<super::input::ColorInput>),
    DateInput(VChild<super::input::DateInput>),
    DateTimeInput(VChild<super::input::DateTimeInput>),
    EmailInput(VChild<super::input::EmailInput>),
    MonthInput(VChild<super::input::MonthInput>),
    IntegerInput(VChild<super::input::IntegerInput>),
    PositiveIntegerInput(VChild<super::input::PositiveIntegerInput>),
    FloatInputInput(VChild<super::input::FloatInput>),
    PhoneInput(VChild<super::input::PhoneInput>),
    PasswordInput(VChild<super::input::PasswordInput>),
    TextInput(VChild<super::input::TextInput>),
    TimeInput(VChild<super::input::TimeInput>),
    UrlInput(VChild<super::input::UrlInput>),
    WeekInput(VChild<super::input::WeekInput>),

    Other(VNode),
}

impl PartialEq for FieldChild {
    fn eq(&self, other: &Self) -> bool {
        use FieldChild::*;
        match (self, other) {
            (ColorInput(self_child), ColorInput(other_child)) => self_child.eq(other_child),
            (DateInput(self_child), DateInput(other_child)) => self_child.eq(other_child),
            (DateTimeInput(self_child), DateTimeInput(other_child)) => self_child.eq(other_child),
            (EmailInput(self_child), EmailInput(other_child)) => self_child.eq(other_child),
            (MonthInput(self_child), MonthInput(other_child)) => self_child.eq(other_child),
            (IntegerInput(self_child), IntegerInput(other_child)) => self_child.eq(other_child),
            (PositiveIntegerInput(self_child), PositiveIntegerInput(other_child)) => {
                self_child.eq(other_child)
            }
            (FloatInputInput(self_child), FloatInputInput(other_child)) => {
                self_child.eq(other_child)
            }
            (PasswordInput(self_child), PasswordInput(other_child)) => self_child.eq(other_child),
            (PhoneInput(self_child), PhoneInput(other_child)) => self_child.eq(other_child),
            (TextInput(self_child), TextInput(other_child)) => self_child.eq(other_child),
            (TimeInput(self_child), TimeInput(other_child)) => self_child.eq(other_child),
            (UrlInput(self_child), UrlInput(other_child)) => self_child.eq(other_child),
            (WeekInput(self_child), WeekInput(other_child)) => self_child.eq(other_child),

            (Other(self_node), Other(other_node)) => self_node.eq(other_node),

            _ => false,
        }
    }
}

impl Into<Html> for FieldChild {
    fn into(self) -> Html {
        use FieldChild::*;
        match self {
            ColorInput(child) => child.into(),
            DateInput(child) => child.into(),
            DateTimeInput(child) => child.into(),
            EmailInput(child) => child.into(),
            MonthInput(child) => child.into(),
            IntegerInput(child) => child.into(),
            PositiveIntegerInput(child) => child.into(),
            FloatInputInput(child) => child.into(),
            PasswordInput(child) => child.into(),
            PhoneInput(child) => child.into(),
            TextInput(child) => child.into(),
            TimeInput(child) => child.into(),
            UrlInput(child) => child.into(),
            WeekInput(child) => child.into(),
            // Textarea
            Other(node) => node,
        }
    }
}
