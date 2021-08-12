use crate::BulmaClass;

pub enum Margin {
    Top(Value),
    Right(Value),
    Bottom(Value),
    Left(Value),
    X(Value),
    Y(Value),
}

pub enum Padding {
    Top(Value),
    Right(Value),
    Bottom(Value),
    Left(Value),
    X(Value),
    Y(Value),
}

pub enum Value {
    /// `0px`
    Zero,
    /// `0.25rem`
    One,
    /// `0.5rem`
    Two,
    /// `0.75rem`
    Three,
    /// `1rem`
    Four,
    /// `1.5rem`
    Five,
    /// `3rem`
    Six,
}

impl Margin {
    pub const TOP_0: Self = Self::Top(Value::Zero);
    pub const TOP_1: Self = Self::Top(Value::One);
    pub const TOP_2: Self = Self::Top(Value::Two);
    pub const TOP_3: Self = Self::Top(Value::Three);
    pub const TOP_4: Self = Self::Top(Value::Four);
    pub const TOP_5: Self = Self::Top(Value::Five);
    pub const TOP_6: Self = Self::Top(Value::Six);

    pub const RIGHT_0: Self = Self::Right(Value::Zero);
    pub const RIGHT_1: Self = Self::Right(Value::One);
    pub const RIGHT_2: Self = Self::Right(Value::Two);
    pub const RIGHT_3: Self = Self::Right(Value::Three);
    pub const RIGHT_4: Self = Self::Right(Value::Four);
    pub const RIGHT_5: Self = Self::Right(Value::Five);
    pub const RIGHT_6: Self = Self::Right(Value::Six);

    pub const BOTTOM_0: Self = Self::Bottom(Value::Zero);
    pub const BOTTOM_1: Self = Self::Bottom(Value::One);
    pub const BOTTOM_2: Self = Self::Bottom(Value::Two);
    pub const BOTTOM_3: Self = Self::Bottom(Value::Three);
    pub const BOTTOM_4: Self = Self::Bottom(Value::Four);
    pub const BOTTOM_5: Self = Self::Bottom(Value::Five);
    pub const BOTTOM_6: Self = Self::Bottom(Value::Six);

    pub const LEFT_0: Self = Self::Left(Value::Zero);
    pub const LEFT_1: Self = Self::Left(Value::One);
    pub const LEFT_2: Self = Self::Left(Value::Two);
    pub const LEFT_3: Self = Self::Left(Value::Three);
    pub const LEFT_4: Self = Self::Left(Value::Four);
    pub const LEFT_5: Self = Self::Left(Value::Five);
    pub const LEFT_6: Self = Self::Left(Value::Six);

    pub const X_0: Self = Self::X(Value::Zero);
    pub const X_1: Self = Self::X(Value::One);
    pub const X_2: Self = Self::X(Value::Two);
    pub const X_3: Self = Self::X(Value::Three);
    pub const X_4: Self = Self::X(Value::Four);
    pub const X_5: Self = Self::X(Value::Five);
    pub const X_6: Self = Self::X(Value::Six);

    pub const Y_0: Self = Self::Y(Value::Zero);
    pub const Y_1: Self = Self::Y(Value::One);
    pub const Y_2: Self = Self::Y(Value::Two);
    pub const Y_3: Self = Self::Y(Value::Three);
    pub const Y_4: Self = Self::Y(Value::Four);
    pub const Y_5: Self = Self::Y(Value::Five);
    pub const Y_6: Self = Self::Y(Value::Six);
}

impl crate::BulmaClass for Margin {
    fn class(&self) -> &'static str {
        use Margin::*;
        use Value::*;
        match self {
            Top(Zero) => "mt-0",
            Top(One) => "mt-1",
            Top(Two) => "mt-2",
            Top(Three) => "mt-3",
            Top(Four) => "mt-4",
            Top(Five) => "mt-5",
            Top(Six) => "mt-6",
            Right(Zero) => "mr-0",
            Right(One) => "mr-1",
            Right(Two) => "mr-2",
            Right(Three) => "mr-3",
            Right(Four) => "mr-4",
            Right(Five) => "mr-5",
            Right(Six) => "mr-6",
            Bottom(Zero) => "mb-0",
            Bottom(One) => "mb-1",
            Bottom(Two) => "mb-2",
            Bottom(Three) => "mb-3",
            Bottom(Four) => "mb-4",
            Bottom(Five) => "mb-5",
            Bottom(Six) => "mb-6",
            Left(Zero) => "ml-0",
            Left(One) => "ml-1",
            Left(Two) => "ml-2",
            Left(Three) => "ml-3",
            Left(Four) => "ml-4",
            Left(Five) => "ml-5",
            Left(Six) => "ml-6",
            X(Zero) => "mx-0",
            X(One) => "mx-1",
            X(Two) => "mx-2",
            X(Three) => "mx-3",
            X(Four) => "mx-4",
            X(Five) => "mx-5",
            X(Six) => "mx-6",
            Y(Zero) => "my-0",
            Y(One) => "my-1",
            Y(Two) => "my-2",
            Y(Three) => "my-3",
            Y(Four) => "my-4",
            Y(Five) => "my-5",
            Y(Six) => "my-6",
        }
    }
}

impl Padding {
    pub const TOP_0: Self = Self::Top(Value::Zero);
    pub const TOP_1: Self = Self::Top(Value::One);
    pub const TOP_2: Self = Self::Top(Value::Two);
    pub const TOP_3: Self = Self::Top(Value::Three);
    pub const TOP_4: Self = Self::Top(Value::Four);
    pub const TOP_5: Self = Self::Top(Value::Five);
    pub const TOP_6: Self = Self::Top(Value::Six);

    pub const RIGHT_0: Self = Self::Right(Value::Zero);
    pub const RIGHT_1: Self = Self::Right(Value::One);
    pub const RIGHT_2: Self = Self::Right(Value::Two);
    pub const RIGHT_3: Self = Self::Right(Value::Three);
    pub const RIGHT_4: Self = Self::Right(Value::Four);
    pub const RIGHT_5: Self = Self::Right(Value::Five);
    pub const RIGHT_6: Self = Self::Right(Value::Six);

    pub const BOTTOM_0: Self = Self::Bottom(Value::Zero);
    pub const BOTTOM_1: Self = Self::Bottom(Value::One);
    pub const BOTTOM_2: Self = Self::Bottom(Value::Two);
    pub const BOTTOM_3: Self = Self::Bottom(Value::Three);
    pub const BOTTOM_4: Self = Self::Bottom(Value::Four);
    pub const BOTTOM_5: Self = Self::Bottom(Value::Five);
    pub const BOTTOM_6: Self = Self::Bottom(Value::Six);

    pub const LEFT_0: Self = Self::Left(Value::Zero);
    pub const LEFT_1: Self = Self::Left(Value::One);
    pub const LEFT_2: Self = Self::Left(Value::Two);
    pub const LEFT_3: Self = Self::Left(Value::Three);
    pub const LEFT_4: Self = Self::Left(Value::Four);
    pub const LEFT_5: Self = Self::Left(Value::Five);
    pub const LEFT_6: Self = Self::Left(Value::Six);

    pub const X_0: Self = Self::X(Value::Zero);
    pub const X_1: Self = Self::X(Value::One);
    pub const X_2: Self = Self::X(Value::Two);
    pub const X_3: Self = Self::X(Value::Three);
    pub const X_4: Self = Self::X(Value::Four);
    pub const X_5: Self = Self::X(Value::Five);
    pub const X_6: Self = Self::X(Value::Six);

    pub const Y_0: Self = Self::Y(Value::Zero);
    pub const Y_1: Self = Self::Y(Value::One);
    pub const Y_2: Self = Self::Y(Value::Two);
    pub const Y_3: Self = Self::Y(Value::Three);
    pub const Y_4: Self = Self::Y(Value::Four);
    pub const Y_5: Self = Self::Y(Value::Five);
    pub const Y_6: Self = Self::Y(Value::Six);
}

impl crate::BulmaClass for Padding {
    fn class(&self) -> &'static str {
        use Padding::*;
        use Value::*;
        match self {
            Top(Zero) => "pt-0",
            Top(One) => "pt-1",
            Top(Two) => "pt-2",
            Top(Three) => "pt-3",
            Top(Four) => "pt-4",
            Top(Five) => "pt-5",
            Top(Six) => "pt-6",
            Right(Zero) => "pr-0",
            Right(One) => "pr-1",
            Right(Two) => "pr-2",
            Right(Three) => "pr-3",
            Right(Four) => "pr-4",
            Right(Five) => "pr-5",
            Right(Six) => "pr-6",
            Bottom(Zero) => "pb-0",
            Bottom(One) => "pb-1",
            Bottom(Two) => "pb-2",
            Bottom(Three) => "pb-3",
            Bottom(Four) => "pb-4",
            Bottom(Five) => "pb-5",
            Bottom(Six) => "pb-6",
            Left(Zero) => "pl-0",
            Left(One) => "pl-1",
            Left(Two) => "pl-2",
            Left(Three) => "pl-3",
            Left(Four) => "pl-4",
            Left(Five) => "pl-5",
            Left(Six) => "pl-6",
            X(Zero) => "px-0",
            X(One) => "px-1",
            X(Two) => "px-2",
            X(Three) => "px-3",
            X(Four) => "px-4",
            X(Five) => "px-5",
            X(Six) => "px-6",
            Y(Zero) => "py-0",
            Y(One) => "py-1",
            Y(Two) => "py-2",
            Y(Three) => "py-3",
            Y(Four) => "py-4",
            Y(Five) => "py-5",
            Y(Six) => "py-6",
        }
    }
}

impl From<Margin> for yew::Classes {
    fn from(m: Margin) -> Self {
        m.to_yew()
    }
}
impl From<Padding> for yew::Classes {
    fn from(p: Padding) -> Self {
        p.to_yew()
    }
}
