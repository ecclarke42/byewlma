use chrono::ParseError;

use crate::components::prelude::*;
use std::borrow::Cow;

/// An html <input />. All types given [here](https://www.w3schools.com/html/html_form_input_types.asp)
/// are available, except for the following:
///     - radio: Use the `Radio` component
///     - checkbox: Use the `Checkbox` component
///     - button: Use the `Button` component (note that `submit` and `reset` ARE available)
///     - file: use the `File` component
///
pub type Input<T: InputValue> = Pure<PureInput<T>>;

#[derive(Debug, Default, Properties)]
pub struct PureInput<T: InputValue> {
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub label: Option<String>,

    #[prop_or_default]
    pub help: Option<String>,

    #[prop_or_default]
    pub value: Option<T>,

    /// Minimum input value (ignored if not supported)
    #[prop_or_default]
    pub min: Option<T>,

    /// Maximum input value (ignored if not supported)
    #[prop_or_default]
    pub max: Option<T>,

    #[prop_or_default]
    pub on_input: Option<Callback<T::Result>>,
}

impl<T: InputValue> Clone for PureInput<T> {
    fn clone(&self) -> Self {
        let PureInput {
            id,
            class,
            style,
            label,
            help,
            value,
            min,
            max,
            on_input,
        } = self;
        PureInput {
            id: id.clone(),
            class: class.clone(),
            style: style.clone(),
            label: label.clone(),
            help: help.clone(),
            value: value.clone(),
            min: min.clone(),
            max: max.clone(),
            on_input: on_input.clone(),
        }
    }
}

impl<T: InputValue> PartialEq for PureInput<T> {
    fn eq(&self, other: &PureInput<T>) -> bool {
        self.id == other.id
            && self.class == other.class
            && self.style == other.style
            && self.label == other.label
            && self.help == other.help
            && self.value == other.value
            && self.on_input == other.on_input
    }
}

impl<T: InputValue> PureComponent for PureInput<T> {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        class.push("input");

        let min = if T::TYPE.supports_min_max() {
            self.min.as_ref().map(|v| v.view().into())
        } else {
            None
        };

        let max = if T::TYPE.supports_min_max() {
            self.max.as_ref().map(|v| v.view().into())
        } else {
            None
        };

        html! {
            <input
                id={self.id.clone()}
                class={class}
                style={self.style.clone()}
                type={T::TYPE.name()}
                value={self.value.as_ref().map(|v| v.view().into())}
                min={min}
                max={max}
            />
        }
    }
}

/// Any type that implements InputValue can be used in an Input
///
/// Implement this for your own newtypes to control input rendering and transformation
pub trait InputValue: Clone + PartialEq + 'static {
    const TYPE: Type;

    type View: Into<Cow<'static, str>>;
    type Result: 'static;

    fn view(&self) -> Self::View;
    fn transform(input: String) -> Self::Result;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Text(String);
impl InputValue for Text {
    const TYPE: Type = Type::Text;
    type View = String;
    type Result = String;
    fn view(&self) -> Self::View {
        self.0.clone()
    }
    fn transform(input: String) -> Self::Result {
        input
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Password(String);
impl InputValue for Password {
    const TYPE: Type = Type::Password;
    type View = String;
    type Result = String;
    fn view(&self) -> Self::View {
        self.0.clone()
    }
    fn transform(input: String) -> Self::Result {
        input
    }
}

// TODO: type=submit
// TODO: type=reset

pub type Color = palette::Srgb<u8>;
impl InputValue for Color {
    const TYPE: Type = Type::Color;
    type View = String;
    type Result = Result<Self, palette::rgb::FromHexError>;
    fn view(&self) -> Self::View {
        format!("#{:x}", self)
    }
    fn transform(input: String) -> Self::Result {
        use std::str::FromStr;
        palette::rgb::Srgb::<u8>::from_str(&input)
    }
}

pub type Date = chrono::NaiveDate;
const JS_DATE_FMT: &str = "%F"; // ISO 8601, same as "%Y-%m-%d"
impl InputValue for Date {
    const TYPE: Type = Type::Date;
    type View = String;
    type Result = Result<Self, ParseError>;
    fn view(&self) -> Self::View {
        self.format(JS_DATE_FMT).to_string()
    }
    fn transform(input: String) -> Self::Result {
        chrono::NaiveDate::parse_from_str(&input, JS_DATE_FMT)
    }
}

pub type DateTime = chrono::NaiveDateTime;
const JS_DATETIME_FMT: &str = "%Y-%m-%dT%H:%M";
impl InputValue for DateTime {
    const TYPE: Type = Type::DateTime;
    type View = String;
    type Result = Result<Self, ParseError>;
    fn view(&self) -> Self::View {
        self.format(JS_DATETIME_FMT).to_string()
    }
    fn transform(input: String) -> Self::Result {
        chrono::NaiveDateTime::parse_from_str(&input, JS_DATETIME_FMT)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Email(String);
impl InputValue for Email {
    const TYPE: Type = Type::Email;
    type View = String;
    type Result = String;
    fn view(&self) -> Self::View {
        self.0.clone()
    }
    fn transform(input: String) -> Self::Result {
        input
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    /// Note: not supported in IE11 or Safari 9.1
    Color,

    /// Note: not supported in Safari or IE11
    Date,

    /// Note: not supported in Firefox, Safari, or IE12
    DateTime,

    Email,
    Hidden,
    Image,
    Month,
    Number,
    Password,
    Range,
    Reset,
    Search,
    Submit,
    Telephone,
    Text,
    Time,
    Url,
    Week,
}

// TODO: checkbox and radio are different in bulma
//     Checkbox,
//     Radio,

impl Type {
    pub fn name(&self) -> &'static str {
        use Type::*;
        match self {
            Color => "color",
            Date => "date",
            DateTime => "datetime-local",
            Email => "email",
            Hidden => "hidden",
            Image => "image",
            Month => "month",
            Number => "number",
            Password => "password",
            Range => "range",
            Reset => "reset",
            Search => "search",
            Submit => "submit",
            Telephone => "tel",
            Text => "text",
            Time => "time",
            Url => "url",
            Week => "week",
        }
    }

    pub fn supports_min_max(&self) -> bool {
        use Type::*;
        match self {
            Date => true,

            // Image => "image",
            // Month => "month",
            // Number => "number",
            // Password => "password",
            // Range => "range",
            // Reset => "reset",
            // Search => "search",
            // Submit => "submit",
            // Telephone => "tel",
            // Text => "text",
            // Time => "time",
            // Url => "url",
            // Week => "week",
            _ => false,
        }
    }
}
