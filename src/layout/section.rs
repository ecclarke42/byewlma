use crate::innerlude::*;

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct SectionProps {
    #[prop_or_default]
    pub id: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub size: SectionSize,
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

/// Bulma [Section](https://bulma.io/documentation/layout/section/) Layout Element
#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    let mut class = props.class.clone();
    unsafe {
        class.unchecked_push("section");
        match props.size {
            SectionSize::Normal => {}
            SectionSize::Medium => class.unchecked_push("is-medium"),
            SectionSize::Large => class.unchecked_push("is-large"),
        }
    }

    html! {
        <section id={props.id.clone()} class={class} style={props.style.clone()}>
            {for props.children.iter()}
        </section>
    }
}
