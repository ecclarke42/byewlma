use crate::{components::prelude::*, SemanticColor};

pure_props! {
    /// Bulma [Hero](https://bulma.io/documentation/layout/hero/) Layout Element
    pub struct Hero {
        #[prop_or_default]
        pub color: Option<SemanticColor>,

        #[prop_or_default]
        pub size: Option<HeroSize>,

        #[prop_or_default]
        pub header: Option<Html>,

        #[prop_or_default]
        pub footer: Option<Html>,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HeroSize {
    Small,
    Medium,
    Large,
    HalfHeight,
    FullHeight,
    FullHeightWithNavbar,
}

impl PureComponent for PureHero {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        class.push("hero");

        if let Some(color) = &self.color {
            class.push(color.is_class());
        }

        match &self.size {
            None => {}
            Some(HeroSize::Small) => class.push("is-small"),
            Some(HeroSize::Medium) => class.push("is-medium"),
            Some(HeroSize::Large) => class.push("is-large"),
            Some(HeroSize::HalfHeight) => class.push("is-halfheight"),
            Some(HeroSize::FullHeight) => class.push("is-fullheight"),
            Some(HeroSize::FullHeightWithNavbar) => class.push("is-fullheight-with-navbar"),
        }

        let head = if let Some(header) = &self.header {
            html! {<div class="hero-head">{header.clone()}</div>}
        } else {
            html! {}
        };

        let foot = if let Some(footer) = &self.footer {
            html! {<div class="hero-foot">{footer.clone()}</div>}
        } else {
            html! {}
        };

        html! {
            <section id={self.id.clone()} class={class} style={self.style.clone()}>
                { head }
                <div class="hero-body">
                    {for self.children.iter()}
                </div>
                { foot }
            </section>
        }
    }
}
