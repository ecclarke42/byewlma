use crate::{innerlude::*, SemanticColor};

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

impl PureComponent for PureHero {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        unsafe {
            class.unchecked_push("hero");
        }

        if let Some(color) = &self.color {
            class.add(color);
        }

        if let Some(size) = &self.size {
            class.add(size);
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
