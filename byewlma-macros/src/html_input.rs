use std::str::FromStr;

use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Token,
};

pub fn transform_stream(
    attrs: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let Attrs {
        input_type,
        value_type,
        attributes,
    } = parse_macro_input!(attrs as Attrs);
    let syn::DeriveInput {
        attrs: extra_attrs,
        vis,
        ident: type_name,
        generics,
        data,
    } = parse_macro_input!(item as syn::DeriveInput);

    let existing_fields = match data {
        syn::Data::Struct(data) => Some(data),
        _ => None,
    }
    .expect("input must be a struct")
    .fields;

    let component_name = quote::format_ident!("Pure{}", type_name);
    let (html_attr_fields, html_attr_props): (Vec<_>, Vec<_>) = attributes
        .into_iter()
        .map(|attr| (attr.field(value_type.clone()), attr.prop()))
        .unzip();

    proc_macro::TokenStream::from(quote! {
        #(#extra_attrs)*
        #vis type #type_name = Pure<#component_name>;

        #[derive(Debug, Clone, PartialEq, Properties)]
        #vis struct #component_name #generics {
            #[prop_or_default]
            pub id: Option<Cow<'static, str>>,
            #[prop_or_default]
            pub class: Classes,
            #[prop_or_default]
            pub style: Option<Cow<'static, str>>,
            #[prop_or_default]
            pub tab_index: Option<i16>,

            #[prop_or_default]
            pub name: Option<Cow<'static, str>>,
            #[prop_or_default]
            pub value: Option<#value_type>,
            #[prop_or_default]
            pub on_input: Option<Callback<<#value_type as InputValue>::Result>>,
            #[prop_or_default]
            pub on_change: Option<Callback<Option<<#value_type as InputValue>::Result>>>,

            // Yew Input Props
            #[prop_or_default]
            pub color: Option<SemanticColor>,
            #[prop_or_default]
            pub size: Option<Size>,
            #[prop_or_default]
            pub rounded: bool,
            #[prop_or_default]
            pub force_hover: bool,
            #[prop_or_default]
            pub force_focus: bool,

            #[prop_or_default]
            pub is_loading: bool, // TODO: control container
            #[prop_or_default]
            pub is_disabled: bool,
            #[prop_or_default]
            pub is_readonly: bool,
            #[prop_or_default]
            pub is_static: bool,

            // Existing Struct props
            #existing_fields

            // Html Input Attrs
            #(#html_attr_fields)*
        }

        impl #component_name {
            fn classes(&self) -> Classes {
                let mut class = self.class.clone();
                class.push("input");

                if let Some(color) = &self.color {
                    class.push(color.is_class());
                }

                if let Some(size) = &self.size {
                    class.push(size.class());
                }

                if self.rounded {
                    class.push("is-rounded");
                }

                if self.force_hover {
                    class.push("is-hovered");
                }

                if self.force_focus {
                    class.push("is-focused");
                }

                if self.is_loading {
                    class.push("is-loading");
                }

                if self.is_static {
                    class.push("is-static");
                }
                class
            }

            fn common(&self) -> (Option<Cow<'static, str>>, Option<Callback<InputData>>, Option<Callback<ChangeData>>) {
                let value = self.value.as_ref().map(|v| v.to_input_value());
                let on_input = self
                    .on_input
                    .as_ref()
                    .map(|cb| cb.reform(|evt: InputData| #value_type::from_input_value(evt.value)));

                let on_change = self.on_change.as_ref().map(|cb| {
                    cb.reform(|evt: ChangeData| {
                        if let ChangeData::Value(value) = evt {
                            Some(#value_type::from_input_value(value))
                        } else {
                            None
                        }
                    })
                });

                (value, on_input, on_change)
            }
        }

        impl PureComponent for #component_name {
            fn render(&self) -> Html {
                let class = self.classes();
                let (value, on_input, on_change) = self.common();
                html! {
                    <input
                        type={#input_type}

                        id={self.id.clone()}
                        class={class}
                        style={self.style.clone()}
                        tabindex={self.tab_index.map(|x| x.to_string())}

                        name={self.name.clone()}
                        value={value}
                        oninput={on_input}
                        onchange={on_change}
                        disabled={self.is_disabled}
                        readonly={self.is_readonly}

                        #(#html_attr_props)*
                    />
                }
            }
        }

    })
}

#[derive(Debug)]
pub struct Attrs {
    pub input_type: syn::LitStr,
    pub value_type: syn::Type,
    pub attributes: syn::punctuated::Punctuated<HtmlInputAttr, syn::token::Comma>,
}

#[derive(Debug)]
pub struct HtmlInputAttr {
    ident: syn::Ident,
    kind: HtmlInputAttrKind,
}

#[derive(Debug)]
pub enum HtmlInputAttrKind {
    // Accept (only file)
    // Alt (only image)
    Autocomplete,
    Autofocus,
    // Capture (only file)
    // Checked (only radio/checkbox)
    // Dirname
    // Disabled (via bulma)
    Form,
    // FormAction (only image/submit... and following)
    // FormEncType
    // FormMethod
    // FormNoValidate
    // FormTarget
    // Height (only image)
    // Id (via common)
    InputMode,
    List,
    Max,
    MaxLength,
    Min,
    MinLength,
    Multiple,
    // Name (on all, so common)
    Pattern,
    Placeholder,
    // Readonly (via bulma)
    Required,
    // Size (redundant to bulma)
    // Src (only image)
    Step(syn::Type),
    /* Title ? (global tooltip)
     * Value (via common)
     * Width (only image) */
}

impl HtmlInputAttr {
    fn field(&self, value_type: syn::Type) -> proc_macro2::TokenStream {
        use HtmlInputAttrKind::*;
        match &self.kind {
            Autocomplete => quote! {
                /// The [`autocomplete`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#attr-autocomplete) attribute
                ///
                /// For more detail, see https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/autocomplete
                #[prop_or_default]
                pub autocomplete: Option<attributes::Autocomplete>,
            },

            Autofocus => quote! {
                /// The [`autofocus`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#attr-autofocus) attribute
                #[prop_or_default]
                pub autofocus: bool,
            },

            Form => quote! {
                /// The [`form`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#attr-form) attribute
                ///
                /// Must match the `id` of a `<form>` element
                #[prop_or_default]
                pub form_id: Option<Cow<'static, str>>,
            },

            InputMode => quote! {
                /// The [`inputmode`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#attr-inputmode) attribute
                ///
                /// Browser keyboard hint
                #[prop_or_default]
                pub input_mode_hint: Option<attributes::ModeHint>,
            },

            List => quote! {
                /// The [`list`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/text#attr-list) attribute
                ///
                /// See also [`<datalist>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/datalist)
                #[prop_or_default]
                pub datalist_id: Option<Cow<'static, str>>,
            },

            Max => quote! {
                /// The [`max`]((https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#attr-max) attribute
                #[prop_or_default]
                pub max: Option<#value_type>,
            },

            MaxLength => quote! {
                /// The [`maxlength`]((https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#attr-maxlength) attribute
                #[prop_or_default]
                pub max_length: Option<u32>,
            },

            Min => quote! {
                /// The [`min`]((https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#attr-min) attribute
                #[prop_or_default]
                pub min: Option<#value_type>,
            },

            MinLength => quote! {
                /// The [`minlength`]((https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#attr-minlength) attribute
                #[prop_or_default]
                pub min_length: Option<u32>,
            },

            Multiple => quote! {
                /// The [`multiple`]((https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#attr-multiple) attribute
                #[prop_or_default]
                pub multiple: bool,
            },

            Pattern => quote! {
                /// The [`pattern`]((https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#attr-pattern) attribute
                #[prop_or_default]
                pub pattern: Option<Cow<'static, str>>,
            },

            Placeholder => quote! {
                /// The [`placeholder`]((https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#attr-placeholder) attribute
                #[prop_or_default]
                pub placeholder: Option<Cow<'static, str>>,
            },

            Required => quote! {
                /// The [`required`]((https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#attr-required) attribute
                #[prop_or_default]
                pub required: bool,
            },

            Step(inner) => quote! {
                /// The [`step`]((https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#attr-step) attribute
                ///
                /// Note, for
                #[prop_or_default]
                pub step: Option<attributes::Step<#inner>>,
            },
        }
    }

    fn prop(&self) -> proc_macro2::TokenStream {
        use HtmlInputAttrKind::*;
        match self.kind {
            Autocomplete => quote! {
                autocomplete={self.autocomplete.map(|x| x.value())}
            },
            Autofocus => quote! {
                autofocus={self.autofocus.clone()}
            },
            Form => quote! {
                form={self.form_id.clone()}
            },
            InputMode => quote! {
                inputmode={self.input_mode_hint.map(|x| x.value())}
            },
            List => quote! {
                list={self.datalist_id.clone()}
            },
            Max => quote! {
                max={self.max.map(|x| x.to_input_value())}
            },
            MaxLength => quote! {
                maxlength={self.max_length.map(|x| x.to_string())}
            },
            Min => quote! {
                min={self.min.map(|x| x.to_input_value())}
            },
            MinLength => quote! {
                minlength={self.min_length.map(|x| x.to_string())}
            },
            Multiple => quote! {
                multiple={self.multiple}
            },
            Pattern => quote! {
                pattern={self.pattern.clone()}
            },
            Placeholder => quote! {
                placeholder={self.placeholder.clone()}
            },
            Required => quote! {
                required={self.required}
            },
            Step(_) => quote! {
                step={self.step.map(|s| s.value())}
            },
        }
    }
}

impl FromStr for HtmlInputAttrKind {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "autocomplete" => Ok(HtmlInputAttrKind::Autocomplete),
            "autofocus" => Ok(HtmlInputAttrKind::Autofocus),
            "form" => Ok(HtmlInputAttrKind::Form),
            "inputmode" => Ok(HtmlInputAttrKind::InputMode),
            "list" => Ok(HtmlInputAttrKind::List),
            "max" => Ok(HtmlInputAttrKind::Max),
            "maxlength" => Ok(HtmlInputAttrKind::MaxLength),
            "min" => Ok(HtmlInputAttrKind::Min),
            "minlength" => Ok(HtmlInputAttrKind::MinLength),
            "multiple" => Ok(HtmlInputAttrKind::Multiple),
            "pattern" => Ok(HtmlInputAttrKind::Pattern),
            "placeholder" => Ok(HtmlInputAttrKind::Placeholder),
            "required" => Ok(HtmlInputAttrKind::Required),
            "step" => Ok(HtmlInputAttrKind::Step(syn::Type::Verbatim(quote! { u32 }))),
            other => Err(other.to_string()),
        }
    }
}

pub enum HtmlInputType {
    Color,
    Date,
    DateTimeLocal,
    Email,
    Hidden,
    Month,
    Number,
    Password,
    Phone,
    Text,
    Time,
    Url,
    Week,
}

impl FromStr for HtmlInputType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "color" => Ok(HtmlInputType::Color),
            "date" => Ok(HtmlInputType::Date),
            "datetime-local" => Ok(HtmlInputType::DateTimeLocal),
            "email" => Ok(HtmlInputType::Email),
            "hidden" => Ok(HtmlInputType::Hidden),
            "month" => Ok(HtmlInputType::Month),
            "number" => Ok(HtmlInputType::Number),
            "password" => Ok(HtmlInputType::Password),
            "tel" => Ok(HtmlInputType::Phone),
            "text" => Ok(HtmlInputType::Text),
            "time" => Ok(HtmlInputType::Time),
            "url" => Ok(HtmlInputType::Url),
            "week" => Ok(HtmlInputType::Week),

            other => Err(other.to_string()),
        }
    }
}

impl Parse for Attrs {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        input.parse::<Token![type]>().expect("expected \"type\"");
        input.parse::<Token![=]>().unwrap();
        let input_type = input.parse::<syn::LitStr>().unwrap();
        let _input_kind =
            HtmlInputType::from_str(&input_type.value()).expect("Unknown html input type");
        input.parse::<Token![,]>().unwrap();
        assert_eq!(input.parse::<syn::Ident>().unwrap().to_string(), "value");
        input.parse::<Token![=]>().unwrap();
        let value_type = input.parse().unwrap();
        input.parse::<Token![,]>().unwrap();
        assert_eq!(
            input.parse::<syn::Ident>().unwrap().to_string(),
            "attributes"
        );
        input.parse::<Token![=]>().unwrap();

        let attribute_input;
        syn::bracketed!(attribute_input in input);
        let attributes = attribute_input
            .parse_terminated(HtmlInputAttr::parse)
            .unwrap();

        Ok(Attrs {
            input_type,
            value_type,
            attributes,
        })
    }
}

impl Parse for HtmlInputAttr {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let ident = input.parse::<syn::Ident>().unwrap();
        let mut kind =
            HtmlInputAttrKind::from_str(&ident.to_string()).expect("Unknown input attribute");

        if let HtmlInputAttrKind::Step(ref mut default) = kind {
            if input.peek(syn::token::Paren) {
                let inner;
                syn::parenthesized!(inner in input);
                if let Ok(ty) = inner.parse::<syn::Type>() {
                    *default = ty;
                }
            }
        }

        Ok(HtmlInputAttr { ident, kind })
    }
}
