use derive_more::From;
use yew::virtual_dom::{VChild, VNode};

use crate::{
    components::{icon::IconProps, Icon},
    innerlude::*,
};

#[derive(Debug, Default, Clone, PartialEq, Properties)]
pub struct ControlProps {
    #[prop_or_default]
    pub id: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub class: Classes,

    // TODO
    // #[prop_or_default]
    // pub tag: Option<Cow<'static, str>>,
    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,

    // TODO: should only be one?
    #[prop_or_default]
    pub children: Children, // ChildrenRenderer<ControlChild>,

    #[prop_or_default]
    pub left_icon: Option<IconProps>,

    #[prop_or_default]
    pub right_icon: Option<IconProps>,

    #[prop_or_default]
    pub expand: bool,
}

// TODO: make icons children? (max 3, <i><input><i>)

/// Bulma [Control](https://bulma.io/documentation/form/general/) Element
#[function_component(Control)]
pub fn control(props: &ControlProps) -> Html {
    let mut class = props.class.clone();
    unsafe {
        class.unchecked_push("control");
    }

    if props.expand {
        unsafe { class.unchecked_push("is-expanded") }

        // TODO: if child is select, use select "is-fullwidth"
    }

    let left_icon = if let Some(icon_props) = &props.left_icon {
        let mut icon_props = icon_props.clone();
        unsafe {
            class.unchecked_push("has-icons-left");
            icon_props.class.unchecked_push("is-left");
        }
        // TODO: Icon props/size?

        html! { <Icon ..icon_props />
            // <span class="icon is-small is-left"> // TODO: icon size? and color?
            //     <i class={classes!("fas", icon_kind.name())}/>
            // </span>
        }
    } else {
        html! {}
    };

    let right_icon = if let Some(icon_props) = &props.right_icon {
        let mut icon_props = icon_props.clone();
        unsafe {
            class.unchecked_push("has-icons-right");
            icon_props.class.unchecked_push("is-right");
        }
        html! { <Icon ..icon_props /> }
    } else {
        html! {}
    };

    html! {
        <div id={props.id.clone()} class={class} style={props.style.clone()}>
            { for props.children.iter() }
            { left_icon }
            { right_icon }
        </div>
    }
}

// TODO: just use super::input::Input somehow?
#[derive(Clone, From)]
pub enum ControlChild {
    ColorInput(VChild<super::ColorInput>),
    DateInput(VChild<super::DateInput>),
    DateTimeInput(VChild<super::DateTimeInput>),
    EmailInput(VChild<super::EmailInput>),
    MonthInput(VChild<super::MonthInput>),
    IntegerInput(VChild<super::IntegerInput>),
    PositiveIntegerInput(VChild<super::PositiveIntegerInput>),
    FloatInput(VChild<super::FloatInput>),
    PhoneInput(VChild<super::PhoneInput>),
    PasswordInput(VChild<super::PasswordInput>),
    TextInput(VChild<super::TextInput>),
    TimeInput(VChild<super::TimeInput>),
    UrlInput(VChild<super::UrlInput>),
    WeekInput(VChild<super::WeekInput>),

    // TODO: just fields, not controls?
    // Checkbox(VChild<super::Checkbox>),
    // RadioButton(VChild<super::RadioButton<T>>),
    // Textarea(VChild<super::TextArea>),

    // TODO: select
    Button(VChild<crate::components::Button>),
    Icon(VChild<crate::components::Icon>),

    Other(VNode),
}

impl PartialEq for ControlChild {
    fn eq(&self, other: &Self) -> bool {
        use ControlChild::*;
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
            (FloatInput(self_child), FloatInput(other_child)) => self_child.eq(other_child),
            (PasswordInput(self_child), PasswordInput(other_child)) => self_child.eq(other_child),
            (PhoneInput(self_child), PhoneInput(other_child)) => self_child.eq(other_child),
            (TextInput(self_child), TextInput(other_child)) => self_child.eq(other_child),
            (TimeInput(self_child), TimeInput(other_child)) => self_child.eq(other_child),
            (UrlInput(self_child), UrlInput(other_child)) => self_child.eq(other_child),
            (WeekInput(self_child), WeekInput(other_child)) => self_child.eq(other_child),

            // (Checkbox(self_child), Checkbox(other_child)) => self_child.eq(other_child),
            // RadioButton(VChild<super::RadioButton<T>>),
            // (Textarea(self_child), Textarea(other_child)) => self_child.eq(other_child),
            (Button(self_child), Button(other_child)) => self_child.eq(other_child),
            (Icon(self_child), Icon(other_child)) => self_child.eq(other_child),

            (Other(self_node), Other(other_node)) => self_node.eq(other_node),

            _ => false,
        }
    }
}

impl From<ControlChild> for Html {
    fn from(child: ControlChild) -> Self {
        use ControlChild::*;
        match child {
            ColorInput(child) => child.into(),
            DateInput(child) => child.into(),
            DateTimeInput(child) => child.into(),
            EmailInput(child) => child.into(),
            MonthInput(child) => child.into(),
            IntegerInput(child) => child.into(),
            PositiveIntegerInput(child) => child.into(),
            FloatInput(child) => child.into(),
            PasswordInput(child) => child.into(),
            PhoneInput(child) => child.into(),
            TextInput(child) => child.into(),
            TimeInput(child) => child.into(),
            UrlInput(child) => child.into(),
            WeekInput(child) => child.into(),

            // Checkbox(child) => child.into(),
            // RadioButton(VChild<super::RadioButton<T>>),
            // Textarea(child) => child.into(),
            Button(child) => child.into(),
            Icon(child) => child.into(),

            Other(node) => node,
        }
    }
}
