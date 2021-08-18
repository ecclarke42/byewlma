use crate::innerlude::*;

pure_props! {
    /// Bulma [Section](https://bulma.io/documentation/layout/section/) Layout Element
    pub struct Section {
        #[prop_or_default]
        pub size: SectionSize,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SectionSize {
    Normal,
    Medium,
    Large,
}

impl Default for SectionSize {
    fn default() -> Self {
        Self::Normal
    }
}

impl PureComponent for PureSection {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        unsafe {
            class.unchecked_push("section");
            match self.size {
                SectionSize::Normal => {}
                SectionSize::Medium => class.unchecked_push("is-medium"),
                SectionSize::Large => class.unchecked_push("is-large"),
            }
        }

        html! {
            <section id={self.id.clone()} class={class} style={self.style.clone()}>
                {for self.children.iter()}
            </section>
        }
    }
}
