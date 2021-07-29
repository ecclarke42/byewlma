//! A Raw Input Element, with all available html attributes
//!
//! In most cases, you probably want a Field (TextField, PasswordField, etc)
//!
//! The following input types are not supported:
//!     - radio: Use the `Radio` component
//!     - checkbox: Use the `Checkbox` component
//!     - button: Use the `Button` component (note that `submit` and `reset` ARE available)
//!     - file: use the `File` component
//!     - search: use a normal Text input
//!     - image: not supported. Use `File`, maybe?
//!     - submit: Use the `Button` component with ButtonType::Submit
//!     - reset: Use the `Button` component with ButtonType::Reset

use crate::components::prelude::*;
use byewlma_macros::html_input;
use chrono::Datelike;
use std::{borrow::Cow, str::FromStr};
use yew::Properties;

pub trait InputValue {
    type Result;
    fn to_input_value(&self) -> std::borrow::Cow<'static, str>;
    fn from_input_value(value: String) -> Self::Result;
}

impl InputValue for String {
    type Result = Self;
    fn to_input_value(&self) -> std::borrow::Cow<'static, str> {
        self.clone().into()
    }
    fn from_input_value(value: String) -> Self::Result {
        value
    }
}

impl InputValue for i32 {
    type Result = Result<Self, std::num::ParseIntError>;
    fn to_input_value(&self) -> std::borrow::Cow<'static, str> {
        self.to_string().into()
    }
    fn from_input_value(value: String) -> Self::Result {
        value.parse()
    }
}
impl InputValue for u32 {
    type Result = Result<Self, std::num::ParseIntError>;
    fn to_input_value(&self) -> std::borrow::Cow<'static, str> {
        self.to_string().into()
    }
    fn from_input_value(value: String) -> Self::Result {
        value.parse()
    }
}

impl InputValue for f32 {
    type Result = Result<Self, std::num::ParseFloatError>;
    fn to_input_value(&self) -> std::borrow::Cow<'static, str> {
        self.to_string().into()
    }
    fn from_input_value(value: String) -> Self::Result {
        value.parse()
    }
}

pub mod attributes {

    // TODO: https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/autocomplete
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Autocomplete {}
    impl Autocomplete {
        pub fn value(&self) -> &'static str {
            ""
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum ModeHint {
        None,
        Text,
        Phone,
        Url,
        Email,
        Numeric,
        Decimal,
        Search,
    }
    impl ModeHint {
        pub fn value(&self) -> &'static str {
            use ModeHint::*;
            match self {
                None => "none",
                Text => "text",
                Phone => "tel",
                Url => "url",
                Email => "email",
                Numeric => "numeric",
                Decimal => "decimal",
                Search => "search",
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Step<T: ToString> {
        Number(T),
        Any,
    }
    impl<T: ToString> Step<T> {
        pub fn value(&self) -> String {
            match self {
                Step::Number(value) => value.to_string(),
                Step::Any => "any".to_string(),
            }
        }
    }
}

// pub use attributes::Autocomplete;

/// An html `<input type="color"/>` Element
#[html_input(
    type = "color",
    value = ColorValue,
    attributes = [
        autocomplete,
        list,
    ]
)]
pub struct ColorInput;
pub type ColorValue = palette::Srgb<u8>;
impl InputValue for ColorValue {
    type Result = Result<Self, palette::rgb::FromHexError>;
    fn to_input_value(&self) -> std::borrow::Cow<'static, str> {
        format!("#{:x}", self).into()
    }
    fn from_input_value(value: String) -> Self::Result {
        palette::rgb::Srgb::<u8>::from_str(&value)
    }
}

/// An html [`<input type="date" />`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/date) Element
///
/// Not supported in IE, but it will "degrade to text" and malformed inputs
/// will be captured in the `DateValue::from_input_value` result
///
/// Step parameter is given in days
#[html_input(
    type = "date",
    value = DateValue,
    attributes = [
        autocomplete,
        list,
        // readonly,
        min,
        max,
        step
    ]
)]
pub struct DateInput;
pub type DateValue = chrono::NaiveDate;
const JS_DATE_FMT: &str = "%F"; // ISO 8601, same as "%Y-%m-%d"
impl InputValue for DateValue {
    type Result = Result<Self, chrono::ParseError>;
    fn to_input_value(&self) -> std::borrow::Cow<'static, str> {
        self.format(JS_DATE_FMT).to_string().into()
    }
    fn from_input_value(value: String) -> Self::Result {
        chrono::NaiveDate::parse_from_str(&value, JS_DATE_FMT)
    }
}

/// An html [`<input type="datetime-local" />`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/datetime-local) Element
///
/// Not supported in IE or Firefox, but like "date" will degrade to text
///
/// Step parameter is given in seconds
#[html_input(
    type = "datetime-local",
    value = DateTimeValue,
    attributes = [
        autocomplete,
        list,
        // readonly,
        min,
        max,
        step
    ]
)]
pub struct DateTimeInput;
pub type DateTimeValue = chrono::NaiveDateTime;
const JS_DATETIME_FMT: &str = "%Y-%m-%dT%H:%M";
impl InputValue for DateTimeValue {
    type Result = Result<Self, chrono::ParseError>;
    fn to_input_value(&self) -> std::borrow::Cow<'static, str> {
        self.format(JS_DATETIME_FMT).to_string().into()
    }
    fn from_input_value(value: String) -> Self::Result {
        chrono::NaiveDateTime::parse_from_str(&value, JS_DATETIME_FMT)
    }
}

/// An html [`<input type="email" />`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/email) Element
#[html_input(
    type = "email",
    value = String,
    attributes = [
        autocomplete,
        list,
        maxlength,
        minlength,
        multiple,
        pattern,
        placeholder,
        // readonly
        required,
    ]
)]
pub struct EmailInput;

/// An html [`<input type="hidden" />`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/hidden) Element
#[html_input(type = "hidden", value = String,attributes = [autocomplete])]
pub struct HiddenInput;

/// An html [`<input type="month" />`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/datetime-local) Element
///
/// Not supported in IE, Firefox, or Safari, but like "date" will degrade to text
///
/// Step parameter is given in months
#[html_input(
    type = "month",
    value = MonthValue,
    attributes = [
        autocomplete,
        list,
        // readonly,
        min,
        max,
        step
    ]
)]
pub struct MonthInput;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MonthValue(chrono::Month, i32);
// const JS_MONTH_FMT: &str = "%Y-%m";
impl InputValue for MonthValue {
    type Result = Result<Self, MonthParseError>;
    fn to_input_value(&self) -> std::borrow::Cow<'static, str> {
        format!("{}-{}", self.1, self.0.number_from_month()).into()
    }
    fn from_input_value(value: String) -> Self::Result {
        // TODO: better way to do this? It would be nice to return a chrono error
        let (year, month) = value.split_once('-').ok_or(MonthParseError::Format)?;
        let year = year.parse::<i32>()?;
        let month = chrono::Month::from_str(month).map_err(|_| MonthParseError::ParseMonth)?;
        Ok(MonthValue(month, year))
    }
}
#[derive(Debug, thiserror::Error)]
pub enum MonthParseError {
    #[error("didn't receive the format \"%Y-%m\"")]
    Format,
    #[error("failed to parse year as integer: {0}")]
    ParseYear(#[from] std::num::ParseIntError),
    // #[error("failed to parse month")]
    // ParseMonth(#[from] chrono::ParseMonthError),
    #[error("failed to parse month")]
    ParseMonth,
}

// TODO
// pub struct IntegerInput

/// An html [`<input type="number" />`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/email) Element
/// for unsigned integers
#[html_input(
    type = "number",
    value = u32,
    attributes = [
        list,
        max,
        min,
        placeholder,
        required,
        step,
    ]
)]
pub struct PositiveIntegerInput;

/// An html [`<input type="number" />`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/email) Element
/// for signed integers
#[html_input(
    type = "number",
    value = i32,
    attributes = [
        list,
        max,
        min,
        placeholder,
        required,
        step,
    ]
)]
pub struct IntegerInput;

/// An html [`<input type="number" />`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/email) Element
/// for floats
#[html_input(
    type = "number",
    value = f32,
    attributes = [
        list,
        max,
        min,
        placeholder,
        required,
        step(f32),
    ]
)]
pub struct FloatInput;

/// An html [`<input type="password" />`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/password) Element
#[html_input(
    type = "password",
    value = String,
    attributes = [
        autocomplete,
        inputmode,
        maxlength,
        minlength,
        pattern,
        placeholder,
        // readonly
        required,
    ]
)]
pub struct PasswordInput;

// TODO: Range (autocomplete, list, max, min, step)

/// An html [`<input type="tel" />`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/tel) Element
#[html_input(
    type = "tel",
    value = String,
    attributes = [
        autocomplete,
        list,
        maxlength,
        minlength,
        pattern,
        placeholder,
        // readonly
    ]
)]
pub struct PhoneInput;

/// An html [`<input type="text" />`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/text) Element
#[html_input(
    type = "text",
    value = String,
    attributes = [
        autocomplete,
        list,
        maxlength,
        minlength,
        pattern,
        placeholder,
        // readonly
        required,
        // TODO: spellcheck
    ]
)]
pub struct TextInput;

/// An html [`<input type="time" />`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/time) Element
///
/// Not supported in IE, but like "date" will degrade to text
///
/// Step parameter is given in seconds
#[html_input(
    type = "time",
    value = TimeValue,
    attributes = [
        autocomplete,
        list,
        // readonly,
        min,
        max,
        step
    ]
)]
pub struct TimeInput;
pub type TimeValue = chrono::NaiveTime;
const JS_TIME_FMT: &str = "%H:%M";
impl InputValue for TimeValue {
    type Result = Result<Self, chrono::ParseError>;
    fn to_input_value(&self) -> std::borrow::Cow<'static, str> {
        self.format(JS_TIME_FMT).to_string().into()
    }
    fn from_input_value(value: String) -> Self::Result {
        chrono::NaiveTime::parse_from_str(&value, JS_TIME_FMT)
    }
}

/// An html [`<input type="url" />`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/url) Element
#[html_input(
    type = "url",
    value = String,
    attributes = [
        autocomplete,
        list,
        maxlength,
        minlength,
        pattern,
        placeholder,
        // readonly
        required,
    ]
)]
pub struct UrlInput;

/// An html [`<input type="week" />`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/week) Element
///
/// Not supported in IE, but like "date" will degrade to text
///
/// Step parameter is given in weeks
#[html_input(
    type = "week",
    value = WeekValue,
    attributes = [
        autocomplete,
        list,
        min,
        max,
        step
    ]
)]
pub struct WeekInput;
pub type WeekValue = chrono::IsoWeek;
// const JS_WEEK_FMT: &str = "%Y-W%U";
impl InputValue for WeekValue {
    type Result = Result<Self, WeekParseError>;
    fn to_input_value(&self) -> std::borrow::Cow<'static, str> {
        format!("{}-W{}", self.year(), self.week()).into()
    }
    fn from_input_value(value: String) -> Self::Result {
        // TODO: better way to do this? It would be nice to return a chrono error
        let (year, week) = value.split_once("-W").ok_or(WeekParseError::Format)?;
        let year = year.parse::<i32>().map_err(WeekParseError::ParseYear)?;
        let week = week.parse::<u32>().map_err(WeekParseError::ParseWeek)?;
        Ok(chrono::NaiveDate::from_isoywd(year, week, chrono::Weekday::Mon).iso_week())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WeekParseError {
    #[error("didn't receive the format \"%Y-W%U\"")]
    Format,
    #[error("failed to parse year as integer: {0}")]
    ParseYear(std::num::ParseIntError),
    #[error("failed to parse week as integer: {0}")]
    ParseWeek(std::num::ParseIntError),
}
