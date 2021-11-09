use crate::innerlude::*;

pub type Size = ColumnSize;
pub type Offset = ColumnOffset;
pub type Gap = ColumnGap;

// TODO: responsiveness: https://bulma.io/documentation/columns/responsiveness/

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct ColumnsProps {
    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: ChildrenWithProps<Column>,

    #[prop_or_default]
    pub gap: Option<ColumnGap>,

    #[prop_or_default]
    pub multiline: bool,

    #[prop_or_default]
    pub center_vertical: bool,

    #[prop_or_default]
    pub center_horizontal: bool,
}

// TODO: responsiveness: https://bulma.io/documentation/columns/gap/#breakpoint-based-column-gaps
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, BulmaClass)]
pub enum ColumnGap {
    #[bulma_class = "is-gapless"]
    None,

    #[bulma_class = "is-1"]
    One,
    #[bulma_class = "is-2"]
    Two,
    #[bulma_class = "is-3"]
    Three,
    #[bulma_class = "is-4"]
    Four,
    #[bulma_class = "is-5"]
    Five,
    #[bulma_class = "is-6"]
    Six,
    #[bulma_class = "is-7"]
    Seven,
    #[bulma_class = "is-8"]
    Eight,
}
impl ColumnGap {
    pub fn is_gapless(&self) -> bool {
        matches!(self, ColumnGap::None)
    }
    pub fn is_variable(&self) -> bool {
        !self.is_gapless()
    }
}

/// Bulma [Columns](https://bulma.io/documentation/columns/) Container
#[function_component(Columns)]
pub fn columns(props: &ColumnsProps) -> Html {
    let mut class = props.class.clone();
    unsafe {
        class.unchecked_push("columns");

        if let Some(gap) = &props.gap {
            class.add(gap);
            if gap.is_variable() {
                class.unchecked_push("is-variable");
            }
        }

        if props.center_vertical {
            class.unchecked_push("is-vcentered");
        }

        if props.center_horizontal {
            class.unchecked_push("is-centered");
        }

        if props.multiline {
            class.unchecked_push("is-multiline");
        }
    }

    html! {
        <div id={props.id.clone()} class={class} style={props.style.clone()}>
            {for props.children.iter()}
        </div>
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct ColumnProps {
    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub size: Option<ColumnSize>,

    #[prop_or_default]
    pub offset: Option<ColumnOffset>,
}

#[derive(Debug, Copy, Clone, PartialEq, BulmaClass)]
pub enum ColumnSize {
    #[bulma_class = "is-narrow"]
    Narrow,

    #[bulma_class = "is-half"]
    Half,
    #[bulma_class = "is-full"]
    Full,

    // Fifths(),
    #[bulma_class = "is-one-fifth"]
    OneFifth,
    #[bulma_class = "is-two-fifths"]
    TwoFifths,
    #[bulma_class = "is-three-fifths"]
    ThreeFifths,
    #[bulma_class = "is-four-fifths"]
    FourFifths,

    // Quarters(),
    #[bulma_class = "is-one-quarter"]
    OneQuarter,
    // Half,
    #[bulma_class = "is-three-quarters"]
    ThreeQuarters,
    // Full,

    // Thirds(),
    #[bulma_class = "is-one-third"]
    OneThird,
    #[bulma_class = "is-two-thirds"]
    TwoThirds,

    // Twelfths(),
    #[bulma_class = "is-1"]
    One,
    #[bulma_class = "is-2"]
    Two,
    #[bulma_class = "is-3"]
    Three,
    #[bulma_class = "is-4"]
    Four,
    #[bulma_class = "is-5"]
    Five,
    #[bulma_class = "is-6"]
    Six,
    #[bulma_class = "is-7"]
    Seven,
    #[bulma_class = "is-8"]
    Eight,
    #[bulma_class = "is-9"]
    Nine,
    #[bulma_class = "is-10"]
    Ten,
    #[bulma_class = "is-11"]
    Eleven,
    #[bulma_class = "is-12"]
    Twelve,
}

// TODO: Dedupe variants with offset?
#[derive(Debug, Copy, Clone, PartialEq, BulmaClass)]
pub enum ColumnOffset {
    #[bulma_class = "is-offset-half"]
    Half,

    #[bulma_class = "is-offset-one-fifth"]
    OneFifth,
    #[bulma_class = "is-offset-two-fifths"]
    TwoFifths,
    #[bulma_class = "is-offset-three-fifths"]
    ThreeFifths,
    #[bulma_class = "is-offset-four-fifths"]
    FourFifths,

    #[bulma_class = "is-offset-one-quarter"]
    OneQuarter,
    #[bulma_class = "is-offset-three-quarters"]
    ThreeQuarters,

    #[bulma_class = "is-offset-one-third"]
    OneThird,
    #[bulma_class = "is-offset-two-thirds"]
    TwoThirds,

    #[bulma_class = "is-offset-1"]
    One,
    #[bulma_class = "is-offset-2"]
    Two,
    #[bulma_class = "is-offset-3"]
    Three,
    #[bulma_class = "is-offset-4"]
    Four,
    #[bulma_class = "is-offset-5"]
    Five,
    #[bulma_class = "is-offset-6"]
    Six,
    #[bulma_class = "is-offset-7"]
    Seven,
    #[bulma_class = "is-offset-8"]
    Eight,
    #[bulma_class = "is-offset-9"]
    Nine,
    #[bulma_class = "is-offset-10"]
    Ten,
    #[bulma_class = "is-offset-11"]
    Eleven,
    #[bulma_class = "is-offset-12"]
    Twelve,
}

/// Bulma [Column](https://bulma.io/documentation/columns/) Element
#[function_component(Column)]
pub fn column(props: &ColumnProps) -> Html {
    let id = props.id.clone();
    let style = props.style.clone();

    let mut class = props.class.clone();
    unsafe {
        class.unchecked_push("column");
    }

    if let Some(size) = &props.size {
        class.add(size);
    }

    if let Some(offset) = &props.offset {
        class.add(offset);
    }

    html! {
        <div {id} {class} {style}>
            {for props.children.iter()}
        </div>
    }
}
