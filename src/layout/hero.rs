use crate::{innerlude::*, SemanticColor};

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct HeroProps {
    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub color: Option<SemanticColor>,

    #[prop_or_default]
    pub size: Option<HeroSize>,

    #[prop_or_default]
    pub header: Option<Html>,

    #[prop_or_default]
    pub footer: Option<Html>,
}

#[derive(Debug, Clone, Copy, PartialEq, BulmaClass)]
pub enum HeroSize {
    #[bulma_class = "is-small"]
    Small,
    #[bulma_class = "is-medium"]
    Medium,
    #[bulma_class = "is-large"]
    Large,
    #[bulma_class = "is-halfheight"]
    HalfHeight,
    #[bulma_class = "is-fullheight"]
    FullHeight,
    #[bulma_class = "is-fullheight-with-navbar"]
    FullHeightWithNavbar,
}

/// Bulma [Hero](https://bulma.io/documentation/layout/hero/) Layout Element
#[function_component(Hero)]
pub fn hero(props: &HeroProps) -> Html {
    let id = props.id.clone();
    let style = props.style.clone();

    let mut class = props.class.clone();
    unsafe {
        class.unchecked_push("hero");
    }

    if let Some(color) = &props.color {
        class.add(color);
    }

    if let Some(size) = &props.size {
        class.add(size);
    }

    let head = if let Some(header) = &props.header {
        html! {<div class="hero-head">{header.clone()}</div>}
    } else {
        html! {}
    };

    let foot = if let Some(footer) = &props.footer {
        html! {<div class="hero-foot">{footer.clone()}</div>}
    } else {
        html! {}
    };

    html! {
        <section {id} {class} {style}>
            { head }
            <div class="hero-body">
                {for props.children.iter()}
            </div>
            { foot }
        </section>
    }
}
