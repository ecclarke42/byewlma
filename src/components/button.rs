use crate::components::prelude::*;

// https://bulma.io/documentation/elements/button/
pub struct Button {
    props: ButtonProps,
}

/// Tag for the button element
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ButtonType {
    /// <a class="button" />
    Anchor { href: Option<String> },
    /// <button class="button" />
    Button,
    /// <input type="submit" class="button" />
    Submit,
    /// <input type="reset" class="button" />
    Reset,
}

impl Default for ButtonType {
    fn default() -> Self {
        ButtonType::Button
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonColor {
    White,
    Light,
    Dark,
    Black,
    Text,
    Ghost,

    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,

    PrimaryLight,
    LinkLight,
    InfoLight,
    SuccessLight,
    WarningLight,
    DangerLight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    Normal,
    Medium,
    Large,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    tag: ButtonType,

    #[prop_or_default]
    color: Option<ButtonColor>,

    #[prop_or_default]
    size: Option<ButtonSize>,

    #[prop_or(false)]
    is_light: bool,

    #[prop_or_default]
    on_click: Option<Callback<()>>,

    #[prop_or_default]
    children: Children,
}

pub enum ButtonMsg {}

impl Component for Button {
    type Properties = ButtonProps;
    type Message = ButtonMsg;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        // match msg {}
        false
    }

    fn view(&self) -> Html {
        match &self.props.tag {
            ButtonType::Anchor { href } => {
                html! {<a class="button" href={href.clone()}>{for self.props.children.iter() }</a>}
            }
            ButtonType::Button => {
                html! {<button class=classes!("button")>{for self.props.children.iter() }</button>}
            }
            ButtonType::Reset => html! {<input type="reset"/>},
            ButtonType::Submit => html! {<input type="submit"/>},
        }
    }
}
