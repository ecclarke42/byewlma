//! A Raw Input Element, with all available html attributes
//!
//! In most cases, you probably want a Field (TextField, PasswordField, etc)
//!
//! The following input types are not supported:
//!     - radio: Use the `Radio` component
//!     - checkbox: Use the `Checkbox` component
//!     - button: Use the `Button` component (note that `submit` and `reset` ARE
//!       available)
//!     - file: use the `File` component
//!     - search: use a normal Text input
//!     - image: not supported. Use `File`, maybe?
//!     - submit: Use the `Button` component with ButtonType::Submit
//!     - reset: Use the `Button` component with ButtonType::Reset

use std::str::FromStr;

use byewlma_macros::html_input;
use chrono::Datelike;
use yew::Properties;

use crate::innerlude::*;

pub trait InputValue {
    type Result;
    fn to_input_value(&self) -> AttrValue;
    fn from_input_value(value: String) -> Self::Result;
}

impl InputValue for String {
    type Result = Self;
    fn to_input_value(&self) -> AttrValue {
        self.clone().into()
    }
    fn from_input_value(value: String) -> Self::Result {
        value
    }
}

impl InputValue for i32 {
    type Result = Result<Self, std::num::ParseIntError>;
    fn to_input_value(&self) -> AttrValue {
        self.to_string().into()
    }
    fn from_input_value(value: String) -> Self::Result {
        value.parse()
    }
}
impl InputValue for u32 {
    type Result = Result<Self, std::num::ParseIntError>;
    fn to_input_value(&self) -> AttrValue {
        self.to_string().into()
    }
    fn from_input_value(value: String) -> Self::Result {
        value.parse()
    }
}

impl InputValue for f32 {
    type Result = Result<Self, std::num::ParseFloatError>;
    fn to_input_value(&self) -> AttrValue {
        self.to_string().into()
    }
    fn from_input_value(value: String) -> Self::Result {
        value.parse()
    }
}

// #[derive(From)]
// pub enum Input {
//     Color(ColorInput),
//     Date(DateInput),
//     DateTime(DateTimeInput),
//     Email(EmailInput),
//     Month(MonthInput),
//     Integer(IntegerInput),
//     PositiveInteger(PositiveIntegerInput),
//     FloatInput(FloatInput),
//     Phone(PhoneInput),
//     Text(TextInput),
//     Time(TimeInput),
//     Url(UrlInput),
//     Week(WeekInput),
// }

pub mod attributes {

    /// Autocomplete Field Options, from https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/autocomplete
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Autocomplete {
        /// Tell the browser to disable autocompletion
        Off,
        /// Allow autocompletion, but with no hints
        On,
        /// A person's full name
        Name,
        /// Name Prefix or title (e.g. Mrs., Mr., Miss, Ms, Dr., ...)
        HonorificPrefix,
        /// Given/First name
        GivenName,
        /// Middle Name
        AdditionalName,
        /// Family/Last name
        FamilyName,
        /// Name suffix (e.g. Jr., "PhD.", "III", ...)
        HonorificSuffix,
        /// Nickname or Handle
        Nickname,

        Email,
        Username,
        /// A new password (as opposed to current)
        NewPassword,
        CurrentPassword,
        OneTimeCode,
        /// Job Title
        OrganizationTitle,
        /// Company or organization name
        Organization,
        /// A street address. This can be multiple lines of text, and should
        /// fully identify the location of the address within its second
        /// administrative level (typically a city or town), but should not
        /// include the city name, ZIP or postal code, or country name.
        StreetAddress,

        /// An individual line of the street address. Should only be present if
        /// "street-address" is not.
        AddressLine1,
        /// An individual line of the street address. Should only be present if
        /// "street-address" is not.
        AddressLine2,
        /// An individual line of the street address. Should only be present if
        /// "street-address" is not.
        AddressLine3,

        /// Finest grained [administrative level](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/autocomplete#administrative_levels_in_addresses)
        AddressLevel4,
        ///Third [administrative level](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/autocomplete#administrative_levels_in_addresses)
        AddressLevel3,
        /// Second [administrative level](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/autocomplete#administrative_levels_in_addresses),
        /// typically the city, town, village, or other locality
        AddressLevel2,
        /// First [administrative level](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/autocomplete#administrative_levels_in_addresses),
        /// typically the province. In the US, a state; in Switzerland, the
        /// canton; in the UK, the post town.
        AddressLevel1,

        /// Country or territory code (i.e. not the full name)
        Country,
        /// Country or territory name
        CountryName,
        PostalCode,

        /// Full Name as printed on or associated with a payment instrument such
        /// as a credit card. Using a full name field is preferred, typically,
        /// over breaking the name into pieces.
        CCName,

        /// A given (first) name as given on a payment instrument like a credit
        /// card.
        CCGivenName,

        /// A middle name as given on a payment instrument or credit card.
        CCAdditionalName,

        /// A family name, as given on a credit card.
        CCFamilyName,

        /// A credit card number or other number identifying a payment method,
        /// such as an account number.
        CCNumber,

        /// A payment method expiration date, typically in the form "MM/YY" or
        /// "MM/YYYY".
        CCExp,

        /// The month in which the payment method expires.
        CCExpMonth,

        /// The year in which the payment method expires.
        CCExpYear,

        /// The security code for the payment instrument; on credit cards, this
        /// is the 3-digit verification number on the back of the card.
        CCCsc,

        /// The type of payment instrument (such as "Visa" or "Master Card").
        CCType,

        /// The currency in which the transaction is to take place.
        TransactionCurrency,

        /// The amount, given in the currency specified by
        /// "transaction-currency", of the transaction, for a payment form.
        TransactionAmount,

        /// A preferred language, given as a valid BCP 47 language tag.
        Language,

        /// A birth date, as a full date.
        Birthday,

        /// The day of the month of a birth date.
        BirthdayDay,

        /// The month of the year of a birth date.
        BirthdayMonth,

        /// The year of a birth date.
        BirthdayYear,

        /// A gender identity (such as "Female", "Fa'afafine", "Male"), as
        /// freeform text without newlines.
        Sex,

        /// A full telephone number, including the country code. If you need to
        /// break the phone number up into its components, you can use these
        /// values for those fields:
        Telephone,

        /// The country code, such as "1" for the United States, Canada, and
        /// other areas in North America and parts of the Caribbean.
        TelephoneCountryCode,

        /// The entire phone number without the country code component,
        /// including a country-internal prefix. For the phone number
        /// "1-855-555-6502", this field's value would be "855-555-6502".
        TelephoneNational,

        /// The area code, with any country-internal prefix applied if
        /// appropriate.
        TelephoneAreaCode,

        /// The phone number without the country or area code. This can be split
        /// further into two parts, for phone numbers which have an exchange
        /// number and then a number within the exchange. For the phone number
        /// "555-6502", use "tel-local-prefix" for "555" and "tel-local-suffix"
        /// for "6502".
        TelephoneLocal,

        /// A telephone extension code within the phone number, such as a room
        /// or suite number in a hotel or an office extension in a company.
        TelephoneExtension,

        /// A URL for an instant messaging protocol endpoint, such as
        /// "xmpp:username@example.net".
        Impp,

        /// A URL, such as a home page or company web site address as
        /// appropriate given the context of the other fields in the form.
        Url,

        ///The URL of an image representing the person, company, or contact
        /// information given in the other fields in the form.
        Photo,
    }
    impl Autocomplete {
        pub fn value(&self) -> &'static str {
            use Autocomplete::*;
            match self {
                Off => "off",
                On => "on",
                Name => "name",
                HonorificPrefix => "honorific-prefix",
                GivenName => "given-name",
                AdditionalName => "additional-name",
                FamilyName => "family-name",
                HonorificSuffix => "honorific-suffix",
                Nickname => "nickname",
                Email => "email",
                Username => "username",
                NewPassword => "new-password",
                CurrentPassword => "current-password",
                OneTimeCode => "one-time-code",
                OrganizationTitle => "organization-title",
                Organization => "organization",
                StreetAddress => "street-address",
                AddressLine1 => "address-line1",
                AddressLine2 => "address-line2",
                AddressLine3 => "address-line3",
                AddressLevel4 => "address-level4",
                AddressLevel3 => "address-level3",
                AddressLevel2 => "address-level2",
                AddressLevel1 => "address-level1",
                Country => "country",
                CountryName => "country-name",
                PostalCode => "postal-code",
                CCName => "cc-name",
                CCGivenName => "cc-given-name",
                CCAdditionalName => "cc-additional-name",
                CCFamilyName => "cc-family-name",
                CCNumber => "cc-number",
                CCExp => "cc-exp",
                CCExpMonth => "cc-exp-month",
                CCExpYear => "cc-exp-year",
                CCCsc => "cc-csc",
                CCType => "cc-type",
                TransactionCurrency => "transaction-currency",
                TransactionAmount => "transaction-amount",
                Language => "language",
                Birthday => "bday",
                BirthdayDay => "bday-day",
                BirthdayMonth => "bday-month",
                BirthdayYear => "bday-year",
                Sex => "sex",
                Telephone => "tel",
                TelephoneCountryCode => "tel-country-code",
                TelephoneNational => "tel-national",
                TelephoneAreaCode => "tel-area-code",
                TelephoneLocal => "tel-local",
                TelephoneExtension => "tel-extension",
                Impp => "impp",
                Url => "url",
                Photo => "photo",
            }
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
    fn to_input_value(&self) -> AttrValue {
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
pub type DateError = chrono::ParseError;
pub type DateResult = Result<DateValue, DateError>;
const JS_DATE_FMT: &str = "%F"; // ISO 8601, same as "%Y-%m-%d"
impl InputValue for DateValue {
    type Result = DateResult;
    fn to_input_value(&self) -> AttrValue {
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
    fn to_input_value(&self) -> AttrValue {
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
/// Not supported in IE, Firefox, or Safari, but like "date" will degrade to
/// text
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
    fn to_input_value(&self) -> AttrValue {
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
pub type PositiveIntegerResult = Result<u32, std::num::ParseIntError>;
// impl InputValue for PositiveIntegerInput {
//     type Result = Result<Self, std::num::ParseIntError>;
//     fn to_input_value(&self) -> AttrValue {

//     }
//     // fn from_input_value(value: String) -> Self::Result {
//     // }
// }

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
pub struct PasswordInput; // TODO: toggle visibility? switch type from "password" to "text"

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
    fn to_input_value(&self) -> AttrValue {
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
    fn to_input_value(&self) -> AttrValue {
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
