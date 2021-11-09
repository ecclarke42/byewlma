use crate::{innerlude::*, Size};

pub type Type = ButtonType;
pub type Color = ButtonColor;

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,

    // Button props
    #[prop_or_default]
    pub tag: ButtonType,
    #[prop_or_default]
    pub color: Option<ButtonColor>,
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub fullwidth: bool,
    #[prop_or_default]
    pub outlined: bool,
    #[prop_or_default]
    pub inverted: bool,
    #[prop_or_default]
    pub rounded: bool,
    #[prop_or_default]
    pub force_hovered: bool,
    /// Note, this does not actually focus the button. It just provides it with
    /// the focus css status
    #[prop_or_default]
    pub force_focused: bool,
    #[prop_or_default]
    pub force_active: bool,
    #[prop_or_default]
    pub loading: bool,
    #[prop_or_default]
    pub r#static: bool,
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub on_click: Option<Callback<()>>,
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

// const RESERVE_CLASSES: usize = 13;

impl ButtonProps {
    fn classes(&self) -> Classes {
        // TODO: reserve?
        let mut classes = self.class.clone();
        classes.push("button");
        if let Some(color) = &self.color {
            use ButtonColor::*;
            match color {
                White => classes.push("is-white"),
                Light => classes.push("is-light"),
                Dark => classes.push("is-dark"),
                Black => classes.push("is-black"),
                Text => classes.push("is-text"),
                Ghost => classes.push("is-ghost"),

                Primary => classes.push("is-primary"),
                Link => classes.push("is-link"),
                Info => classes.push("is-info"),
                Success => classes.push("is-success"),
                Warning => classes.push("is-warning"),
                Danger => classes.push("is-danger"),

                PrimaryLight => {
                    classes.push("is-primary");
                    classes.push("is-light");
                }
                LinkLight => {
                    classes.push("is-link");
                    classes.push("is-light");
                }
                InfoLight => {
                    classes.push("is-info");
                    classes.push("is-light");
                }
                SuccessLight => {
                    classes.push("is-success");
                    classes.push("is-light");
                }
                WarningLight => {
                    classes.push("is-warning");
                    classes.push("is-light");
                }
                DangerLight => {
                    classes.push("is-danger");
                    classes.push("is-light");
                }
            }
        }

        if let Some(size) = self.size {
            classes.push(size.class())
        }

        if self.fullwidth {
            classes.push("is-fullwidth");
        }

        if self.outlined {
            classes.push("is-outlined");
        }

        if self.inverted {
            classes.push("is-inverted");
        }

        if self.rounded {
            classes.push("is-rounded");
        }

        if self.force_hovered {
            classes.push("is-hovered");
        }

        if self.force_focused {
            classes.push("is-focused");
        }

        if self.force_active {
            classes.push("is-active");
        }

        if self.loading {
            classes.push("is-loading");
        }

        if self.r#static {
            classes.push("is-static");
        }

        classes
    }
}

/// Bulma [Button](https://bulma.io/documentation/elements/button/) Element
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let id = props.id.clone();
    let style = props.style.clone();

    let class = props.classes();

    let (tag, ty, href) = match &props.tag {
        ButtonType::Anchor { href } => ("a", None, href.as_ref().cloned()),
        ButtonType::Button => ("button", None, None),
        ButtonType::Reset => ("input", Some("reset"), None),
        ButtonType::Submit => ("button", Some("submit"), None),
    };

    // let disabled = if props.disabled { Some(true) } else { None };
    let onclick = props.on_click.as_ref().map(|cb| cb.reform(|_| ()));

    html! {
        <@{tag}
            {id}
            {class}
            {style}

            disabled={props.disabled} // Todo: check if this renders when false
            {onclick}

            type={ty}
            {href}
        >
            {for props.children.iter()}
        </@>
    }
}
