use std::rc::Rc;

use crate::{
    components::icon::{Icon, IconKind},
    innerlude::*,
};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MenuProps {
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Css Classes to pass along to the root menu element
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: ChildrenWithProps<MenuSection>,
}

/// Bulma [Menu](https://bulma.io/documentation/components/menu/) Component
///
/// A menu is used to wrap any number of MenuSection's
#[function_component(Menu)]
pub fn menu(props: &MenuProps) -> Html {
    let id = props.id.clone();
    let style = props.style.clone();

    let mut class = props.class.clone();
    unsafe {
        class.unchecked_push("menu");
    }

    html! {
        <aside {id} {class} {style}>
        { for props.children.iter() }
        </aside>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CollapsibleMenuProps {
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Css Classes to pass along to the root menu element
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children, // ChildrenWithProps<CollapsibleMenuSection>,

    #[prop_or_default]
    pub is_collapsed: bool,

    /// Indicate that the menu is on the right side of the screen (instead of
    /// left) and flip the arrow on the collapse button
    #[prop_or_default]
    pub is_right: bool,

    /// Fire a callback when the menu is collapsed/expanded
    ///
    /// The collapse/expand button will only be shown if this is Some(_)
    #[prop_or_default]
    pub on_collapse: Option<Callback<bool>>,
}

pub trait MenuState: From<bool> + PartialEq {
    fn collapsed(&self) -> bool;
}

impl MenuState for bool {
    fn collapsed(&self) -> bool {
        *self
    }
}

/// A collapsible menu inspired by the Bulma [`Menu`].
///
/// This includes a collapse button on the bottom that hides the labels of menu
/// items, showing only thier icons.
///
/// Unlike [`Menu`], this component will take the full height of the container it is given
#[function_component(CollapsibleMenu)]
pub fn collapsible_menu<Ctx: MenuState + 'static = bool>(props: &CollapsibleMenuProps) -> Html {
    let id = props.id.clone();
    let style = props.style.clone();

    // TODO: byewlma Column?
    let mut col_class = props.class.clone();
    unsafe {
        // col_class.unchecked_push("column");
        col_class.unchecked_push("menu-collapse-wrapper");
        col_class.unchecked_push("is-flex-direction-column");
        col_class.unchecked_push("is-justify-content-space-between");
    }

    let mut menu_class = Classes::new();
    unsafe {
        menu_class.unchecked_push("menu");
    }

    let on_click = {
        let is_collapsed = props.is_collapsed;
        props
            .on_collapse
            .as_ref()
            .map(|cb| cb.reform(move |_| !is_collapsed))
    };

    let icon = if props.is_collapsed ^ props.is_right {
        html! { <Icon kind={IconKind::AngleDoubleRight} /> }
    } else {
        html! { <Icon kind={IconKind::AngleDoubleLeft} /> }
    };

    html! {
        <ContextProvider<Rc<Ctx>> context={Rc::new(Ctx::from(props.is_collapsed))}>
            <div {id} class={col_class} {style}>
                <aside {menu_class}>
                    { for props.children.iter() }
                </aside>
                <CollapsibleMenuSection<Ctx>>
                    <CollapsibleMenuItem<Ctx> {icon} {on_click}
                        label={"Collapse"}
                        class={classes!("menu-collapse-button")}
                    />
                </CollapsibleMenuSection<Ctx>>
            </div>
        </ContextProvider<Rc<Ctx>>>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MenuSectionProps {
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Css Classes to pass along to the menu section's <ul> list
    /// Classes can be directly passed to the label element when creating it
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub label: Option<Html>,

    #[prop_or_default]
    pub children: Children,
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

    // TODO: Id/style/etc

    html! {
        <>
            { label }
            { children }
        </>
    }
}

#[function_component(CollapsibleMenuSection)]
pub fn collapsible_menu_section<Ctx: MenuState + 'static = bool>(props: &MenuSectionProps) -> Html {
    let state = use_context::<Rc<Ctx>>();
    let is_collapsed = state.as_ref().map(|s| s.collapsed()).unwrap_or_default();

    // if is_collapsed {
    //     return html! { < div class="divider" /> }; // TODO: byewlma component?
    // }

    let label = match (is_collapsed, &props.label) {
        (false, Some(label)) => html! {
            <p class="menu-label">{label.clone()}</p>
        },
        _else => html! {},
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

    // TODO: Id/style/etc

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
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub is_active: bool,

    pub label: Html,

    #[prop_or_default]
    pub on_click: Option<Callback<()>>,

    #[prop_or_default]
    pub children: Children,
}

/// An item in the Bulma [Menu](https://bulma.io/documentation/components/menu/) Component
///
/// MenuItem's can be nested to provide nested lists. Bulma officially only
/// supports two levels of nesting, but it seems to work deeper.
#[function_component(MenuItem)]
pub fn menu_item(props: &MenuItemProps) -> Html {
    let id = props.id.clone();
    let style = props.style.clone();

    let class = props.class.clone();

    let children = if !props.children.is_empty() {
        html! { <ul>{ for props.children.iter() }</ul> }
    } else {
        html! {}
    };

    let onclick = props.on_click.as_ref().map(|cb| cb.reform(|_| ()));

    let mut link_class = Classes::new();
    if props.is_active {
        unsafe {
            link_class.unchecked_push("is-active"); // TODO: replace with BULMA::IsActive;
        }
    }

    html! {
        <li {id} {class} {style}>
            <a class={link_class} {onclick}>
                { props.label.clone() }
            </a>
            { children }
        </li>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CollapsibleMenuItemProps {
    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub is_active: bool,

    // /// Hide this menu item
    // ///
    // /// This renders an empty node, rather than hiding it though css
    // #[prop_or_default]
    // pub is_hidden: bool,
    /// Force this item to collapse (or inherit from the parent menu)
    #[prop_or_default]
    pub is_collapsed: bool,

    #[prop_or_default]
    pub on_click: Option<Callback<()>>,

    pub icon: Html,
    pub label: String, // AttrValue here causes a recursion?

    #[prop_or_default]
    pub children: Children,
}

#[function_component(CollapsibleMenuItem)]
pub fn collapsible_menu_item<Ctx: MenuState + 'static = bool>(
    props: &CollapsibleMenuItemProps,
) -> Html {
    let id = props.id.clone();
    let style = props.style.clone();

    let class = props.class.clone();

    let children = if !props.children.is_empty() {
        html! { <ul>{ for props.children.iter() }</ul> }
    } else {
        html! {}
    };

    let onclick = props.on_click.as_ref().map(|cb| cb.reform(|_| ()));

    let mut link_class = Classes::new();
    if props.is_active {
        unsafe {
            link_class.unchecked_push("is-active"); // TODO: replace with BULMA::IsActive;
        }
    }

    let is_collapsed = props.is_collapsed
        || use_context::<Rc<Ctx>>()
            .map(|s| s.collapsed())
            .unwrap_or_default();

    let mut label_class = Classes::new();
    if is_collapsed {
        unsafe {
            label_class.unchecked_push("is-hidden");
        }
    }

    // let label = props.label.to_string();

    // // if props.is_hidden {
    // //     html! {}
    // // } else {
    html! {
        <li {id} {class} {style}>
            <a class={link_class} {onclick}>
                <span>
                    { props.icon.clone() }
                    <span class={label_class}>
                        { props.label.clone() }
                    </span>
                </span>
            </a>
            { children }
        </li>
    }
    // // }
}
