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
    class: Option<Classes>,

    #[prop_or_default]
    tag: ButtonType,

    #[prop_or_default]
    color: Option<ButtonColor>,

    #[prop_or_default]
    size: Option<ButtonSize>,

    #[prop_or(false)]
    fullwidth: bool,

    #[prop_or(false)]
    outlined: bool,

    #[prop_or(false)]
    inverted: bool,

    #[prop_or(false)]
    rounded: bool,

    #[prop_or(false)]
    force_hovered: bool,

    /// Note, this does not actually focus the button. It just provides it with
    /// the focus css status
    #[prop_or(false)]
    force_focused: bool,

    #[prop_or(false)]
    force_active: bool,

    #[prop_or(false)]
    loading: bool,

    #[prop_or(false)]
    r#static: bool,

    #[prop_or(false)]
    disabled: bool,

    #[prop_or_default]
    on_click: Option<Callback<()>>,

    #[prop_or_default]
    children: Children,
}

const RESERVE_CLASSES: usize = 13;
impl ButtonProps {
    fn classes(&self) -> Classes {
        let mut classes = if let Some(class) = &self.class {
            class.clone()
        } else {
            yew::Classes::with_capacity(RESERVE_CLASSES)
        };

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
            use ButtonSize::*;
            match size {
                Small => classes.push("is-small"),
                Normal => classes.push("is-normal"),
                Medium => classes.push("is-medium"),
                Large => classes.push("is-large"),
            }
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
        let mut attrs = Vec::with_capacity(3);
        let mut node = match &self.props.tag {
            ButtonType::Anchor { href } => {
                if let Some(href) = href {
                    attrs.push(yew::virtual_dom::PositionalAttr::new(
                        "href",
                        href.to_owned(),
                    ));
                }
                yew::virtual_dom::VTag::new("a")
            }
            ButtonType::Button => yew::virtual_dom::VTag::new("button"),
            ButtonType::Reset => {
                let mut node = yew::virtual_dom::VTag::new("input");
                node.kind = Some("reset".into());
                node
            }
            ButtonType::Submit => {
                let mut node = yew::virtual_dom::VTag::new("input");
                node.kind = Some("submit".into());
                node
            }
        };

        // TODO: should we just construct classes as a string?
        attrs.push(yew::virtual_dom::PositionalAttr::new(
            "class",
            self.props.classes().to_string(),
        ));

        if self.props.disabled {
            attrs.push(yew::virtual_dom::PositionalAttr::new_boolean(
                "disabled", true,
            ));
        }

        if let Some(callback) = &self.props.on_click {
            node.listeners
                .push(std::rc::Rc::new(yew::html::onclick::Wrapper::new(
                    callback.reform(|_| ()),
                )))
        }

        if !self.props.children.is_empty() {
            node.add_children(self.props.children.clone());
        }

        node.attributes = yew::virtual_dom::Attributes::Vec(attrs);
        yew::virtual_dom::VNode::VTag(Box::new(node))
    }
}
