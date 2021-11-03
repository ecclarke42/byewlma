use crate::innerlude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MenuProps {
    #[prop_or_default]
    pub id: Option<Cow<'static, str>>,

    /// Css Classes to pass along to the root menu element
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub children: ChildrenWithProps<MenuSection>,
}

/// Bulma [Menu](https://bulma.io/documentation/components/menu/) Component
///
/// A menu is used to wrap any number of MenuSection's
#[function_component(Menu)]
pub fn menu(props: &MenuProps) -> Html {
    let mut class = props.class.clone();
    class.push("menu");
    html! { <aside id={props.id.clone()} class={class} style={props.style.clone()}>{ for props.children.iter() }</aside> }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MenuSectionProps {
    #[prop_or_default]
    pub id: Option<Cow<'static, str>>,

    /// Css Classes to pass along to the menu section's <ul> list
    /// Classes can be directly passed to the label element when creating it
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub label: Option<Html>,

    #[prop_or_default]
    pub children: ChildrenWithProps<MenuItem>,
}

/// An item group in the Bulma [Menu](https://bulma.io/documentation/components/menu/) Component
///
/// A MenuSection can have an optional label (through the `label` property)
/// and any number of MenuItems (which can be nested)
#[function_component(MenuSection)]
pub fn menu_section(props: &MenuSectionProps) -> Html {
    let label = if let Some(label) = props.label.clone() {
        html! {<p class="menu-label">{label}</p>}
    } else {
        html! {}
    };
    let children = if !props.children.is_empty() {
        html! {
            <ul class="menu-list">
                { for props.children.iter() }
            </ul>
        }
    } else {
        html! {}
    };
    html! {
        <>
            { label }
            { children }
        </>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MenuItemProps {
    #[prop_or_default]
    pub id: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub is_active: bool,

    pub label: Html,

    #[prop_or_default]
    pub children: ChildrenWithProps<MenuItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MenuItemAction {
    Click(Callback<()>),

    Link(Cow<'static, str>),

    None,
}

impl Default for MenuItemAction {
    fn default() -> Self {
        Self::None
    }
}

/// An item in the Bulma [Menu](https://bulma.io/documentation/components/menu/) Component
///
/// MenuItem's can be nested to provide nested lists. Bulma officially only
/// supports two levels of nesting, but it seems to work deeper.
#[function_component(MenuItem)]
pub fn menu_item(props: &MenuItemProps) -> Html {
    let children = if !props.children.is_empty() {
        html! { <ul>{ for props.children.iter() }</ul> }
    } else {
        html! {}
    };

    html! {
        <li id={props.id.clone()} class={props.class.clone()} style={props.style.clone()}>
            <a class={classes!(if props.is_active { "is-active" } else { "" })}>
                { props.label.clone() }
            </a>
            { children }
        </li>
    }
}
