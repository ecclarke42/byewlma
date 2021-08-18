use crate::innerlude::*;

/// Bulma [Columns](https://bulma.io/documentation/columns/) Container
pub type Columns = Pure<PureColumns>;

pub type Size = ColumnSize;
pub type Offset = ColumnOffset;
pub type Gap = ColumnGap;

// TODO: responsiveness: https://bulma.io/documentation/columns/responsiveness/

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct PureColumns {
    #[prop_or_default]
    pub id: Option<Cow<'static, str>>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,

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

impl PureComponent for PureColumns {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        unsafe {
            class.unchecked_push("columns");

            if let Some(gap) = &self.gap {
                class.add(gap);
                if gap.is_variable() {
                    class.unchecked_push("is-variable");
                }
            }

            if self.center_vertical {
                class.unchecked_push("is-vcentered");
            }

            if self.center_horizontal {
                class.unchecked_push("is-centered");
            }

            if self.multiline {
                class.unchecked_push("is-multiline");
            }
        }

        html! {
            <div id={self.id.clone()} class={class} style={self.style.clone()}>
                {for self.children.iter()}
            </div>
        }
    }
}

pure_props! {
    /// Bulma [Column](https://bulma.io/documentation/columns/) Element
    pub struct Column {
        #[prop_or_default]
        pub size: Option<ColumnSize>,

        #[prop_or_default]
        pub offset: Option<ColumnOffset>,
    }
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

impl PureComponent for PureColumn {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        unsafe {
            class.unchecked_push("column");
        }

        if let Some(size) = &self.size {
            class.add(size);
        }

        if let Some(offset) = &self.offset {
            class.add(offset);
        }

        html! {
            <div id={self.id.clone()} class={class} style={self.style.clone()}>
                {for self.children.iter()}
            </div>
        }
    }
}
