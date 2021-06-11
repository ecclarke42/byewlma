use crate::components::prelude::*;

/// Bulma [Columns](https://bulma.io/documentation/columns/) Container
pub type Columns = Pure<PureColumns>;

/// Bulma [Column](https://bulma.io/documentation/columns/) Element
pub type Column = Pure<PureColumn>;

// TODO: responsiveness: https://bulma.io/documentation/columns/responsiveness/

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct PureColumns {
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub(crate) children: ChildrenWithProps<Column>,

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
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum ColumnGap {
    None,

    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl PureComponent for PureColumns {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        class.push("columns");

        match self.gap {
            None => {}
            Some(ColumnGap::None) => {
                class.push("is-gapless");
            }
            Some(ColumnGap::One) => {
                class.push("is-variable is-1");
            }
            Some(ColumnGap::Two) => {
                class.push("is-variable is-2");
            }
            Some(ColumnGap::Three) => {
                class.push("is-variable is-3");
            }
            Some(ColumnGap::Four) => {
                class.push("is-variable is-4");
            }
            Some(ColumnGap::Five) => {
                class.push("is-variable is-5");
            }
            Some(ColumnGap::Six) => {
                class.push("is-variable is-6");
            }
            Some(ColumnGap::Seven) => {
                class.push("is-variable is-7");
            }
            Some(ColumnGap::Eight) => {
                class.push("is-variable is-8");
            }
        }

        if self.center_vertical {
            class.push("is-vcentered");
        }

        if self.center_horizontal {
            class.push("is-centered");
        }

        if self.multiline {
            class.push("is-multiline");
        }

        html! {
            <div id={self.id.clone()} class={class} style={self.style.clone()}>
                {for self.children.iter()}
            </div>
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct PureColumn {
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub(crate) children: Children,

    #[prop_or_default]
    pub size: Option<ColumnSize>,

    #[prop_or_default]
    pub offset: Option<ColumnOffset>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ColumnSize {
    Narrow,

    Half,
    Full,

    // Fifths(),
    OneFifth,
    TwoFifths,
    ThreeFifths,
    FourFifths,

    // Quarters(),
    OneQuarter,
    // Half,
    ThreeQuarters,
    // Full,

    // Thirds(),
    OneThird,
    TwoThirds,

    // Twelfths(),
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
}

impl ColumnSize {
    // pub const ONE_THIRD: Self = ColumnSize::Thirds(Thirds::One)
    fn class(&self) -> &'static str {
        use ColumnSize::*;
        match self {
            Narrow => "is-narrow",

            Half => "is-half",
            Full => "is-full",

            OneFifth => "is-one-fifth",
            TwoFifths => "is-two-fifths",
            ThreeFifths => "is-three-fifths",
            FourFifths => "is-four-fifths",

            OneQuarter => "is-one-quarter",
            ThreeQuarters => "is-three-quarters",

            OneThird => "is-one-third",
            TwoThirds => "is-two-thirds",

            One => "is-1",
            Two => "is-2",
            Three => "is-3",
            Four => "is-4",
            Five => "is-5",
            Six => "is-6",
            Seven => "is-7",
            Eight => "is-8",
            Nine => "is-9",
            Ten => "is-10",
            Eleven => "is-11",
            Twelve => "is-12",
        }
    }
}

// TODO: Dedupe variants with offset?
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ColumnOffset {
    Half,

    OneFifth,
    TwoFifths,
    ThreeFifths,
    FourFifths,

    OneQuarter,
    ThreeQuarters,

    OneThird,
    TwoThirds,

    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
}

impl ColumnOffset {
    fn class(&self) -> &'static str {
        use ColumnOffset::*;
        match self {
            Half => "is-offset-half",

            OneFifth => "is-offset-one-fifth",
            TwoFifths => "is-offset-two-fifths",
            ThreeFifths => "is-offset-three-fifths",
            FourFifths => "is-offset-four-fifths",

            OneQuarter => "is-offset-one-quarter",
            ThreeQuarters => "is-offset-three-quarters",

            OneThird => "is-offset-one-third",
            TwoThirds => "is-offset-two-thirds",

            One => "is-offset-1",
            Two => "is-offset-2",
            Three => "is-offset-3",
            Four => "is-offset-4",
            Five => "is-offset-5",
            Six => "is-offset-6",
            Seven => "is-offset-7",
            Eight => "is-offset-8",
            Nine => "is-offset-9",
            Ten => "is-offset-10",
            Eleven => "is-offset-11",
            Twelve => "is-offset-12",
        }
    }
}

impl PureComponent for PureColumn {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        class.push("column");

        if let Some(size) = self.size {
            class.push(size.class());
        }

        if let Some(offset) = self.offset {
            class.push(offset.class());
        }

        html! {
            <div id={self.id.clone()} class={class} style={self.style.clone()}>
                {for self.children.iter()}
            </div>
        }
    }
}
