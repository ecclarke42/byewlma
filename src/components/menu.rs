use crate::innerlude::*;

/// Bulma [Menu](https://bulma.io/documentation/components/menu/) Component
///
/// A menu is used to wrap any number of MenuSection's
pub struct Menu {
    props: MenuProps,
}

/// An item group in the Bulma [Menu](https://bulma.io/documentation/components/menu/) Component
///
/// A MenuSection can have an optional label (through the `label` property)
/// and any number of MenuItems (which can be nested)
pub struct MenuSection {
    props: MenuSectionProps,
}

/// An item in the Bulma [Menu](https://bulma.io/documentation/components/menu/) Component
///
/// MenuItem's can be nested to provide nested lists. Bulma officially only
/// supports two levels of nesting, but it seems to work deeper.
pub struct MenuItem {
    props: MenuItemProps,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MenuProps {
    /// Css Classes to pass along to the root menu element
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub children: ChildrenWithProps<MenuSection>,
}

impl Component for Menu {
    type Properties = MenuProps;
    type Message = ();

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let mut class = self.props.class.clone();
        class.push("menu");
        html! { <aside class={class}>{ for self.props.children.iter() }</aside> }
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MenuSectionProps {
    /// Css Classes to pass along to the menu section's <ul> list
    /// Classes can be directly passed to the label element when creating it
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub label: Option<Html>,

    #[prop_or_default]
    pub children: ChildrenWithProps<MenuItem>,
}

impl Component for MenuSection {
    type Properties = MenuSectionProps;
    type Message = ();

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
        false
    }

    fn view(&self) -> Html {
        let label = if let Some(label) = self.props.label.clone() {
            html! {<p class="menu-label">{label}</p>}
        } else {
            html! {}
        };
        let children = if !self.props.children.is_empty() {
            html! {
                <ul class="menu-list">
                    { for self.props.children.iter() }
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
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MenuItemProps {
    #[prop_or_default]
    pub active: bool,

    #[prop_or_default]
    pub on_click: Option<Callback<()>>,

    pub label: Html,

    #[prop_or_default]
    pub children: ChildrenWithProps<MenuItem>,
}

impl Component for MenuItem {
    type Properties = MenuItemProps;
    type Message = ();

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let children = if !self.props.children.is_empty() {
            html! { <ul>{ for self.props.children.iter() }</ul> }
        } else {
            html! {}
        };
        let onclick = self.props.on_click.as_ref().map(|cb| cb.reform(|_| ()));
        html! {
            <li>
                <a class={if self.props.active {"is-active"} else {""}} onclick={onclick}>
                { self.props.label.clone() }
                </a>
                { children }
            </li>
        }
    }
}
