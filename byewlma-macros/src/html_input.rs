use std::str::FromStr;

use heck::SnakeCase;
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
        ident: component_name,
        generics,
        data,
    } = parse_macro_input!(item as syn::DeriveInput);

    let existing_fields = match data {
        syn::Data::Struct(data) => Some(data),
        _ => None,
    }
    .expect("input must be a struct")
    .fields;

    let (html_attr_fields, html_attr_props): (Vec<_>, Vec<_>) = attributes
        .into_iter()
        .map(|attr| (attr.field(value_type.clone()), attr.prop()))
        .unzip();

    let props_name = quote::format_ident!("{}Props", component_name);
    let fn_name = syn::Ident::new(
        &component_name.to_string().to_snake_case(),
        component_name.span(),
    );
    proc_macro::TokenStream::from(quote! {

        #[derive(Debug, Clone, PartialEq, Properties)]
        #vis struct #props_name #generics {
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
            pub color: Option<crate::SemanticColor>,
            #[prop_or_default]
            pub size: Option<crate::Size>,
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

        impl #props_name {
            fn classes(&self) -> Classes {
                let mut class = self.class.clone();
                unsafe {
                    class.unchecked_push("input");
                    if let Some(color) = &self.color {
                        class.add(color);
                    }
                    if let Some(size) = &self.size {
                        class.add(size);
                    }

                    if self.rounded {
                        class.unchecked_push("is-rounded");
                    }

                    if self.force_hover {
                        class.unchecked_push("is-hovered");
                    }

                    if self.force_focus {
                        class.unchecked_push("is-focused");
                    }

                    if self.is_loading {
                        class.unchecked_push("is-loading");
                    }

                    if self.is_static {
                        class.unchecked_push("is-static");
                    }
                }

                class
            }
        }

        #(#extra_attrs)*
        #[function_component(#component_name)]
        #vis fn #fn_name(props: &#props_name) -> Html {
            let class = props.classes();

            let value = props.value.as_ref().map(|x| x.to_input_value());
            let on_input = props.on_input.as_ref()
                    .map(|cb| cb.reform(|evt: InputEvent| #value_type::from_input_value(evt.target_unchecked_into::<web_sys::HtmlInputElement>().value())));

            let on_change = props.on_change.as_ref().map(|cb| {
                cb.reform(|evt: web_sys::Event| {
                    let value = evt.target_unchecked_into::<web_sys::HtmlInputElement>()
                        .value();

                    if value.is_empty() {
                        None
                    } else {
                        Some(#value_type::from_input_value(value))
                    }
                })
            });

            html! {
                <input
                    type={#input_type}

                    id={props.id.clone()}
                    class={class}
                    style={props.style.clone()}
                    tabindex={props.tab_index.map(|x| x.to_string())}

                    name={props.name.clone()}
                    value={value}
                    oninput={on_input}
                    onchange={on_change}
                    disabled={props.is_disabled}
                    readonly={props.is_readonly}

                    #(#html_attr_props)*
                />
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
    Step(Box<syn::Type>),
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
                autocomplete={props.autocomplete.map(|x| x.value())}
            },
            Autofocus => quote! {
                autofocus={props.autofocus.clone()}
            },
            Form => quote! {
                form={props.form_id.clone()}
            },
            InputMode => quote! {
                inputmode={props.input_mode_hint.map(|x| x.value())}
            },
            List => quote! {
                list={props.datalist_id.clone()}
            },
            Max => quote! {
                max={props.max.map(|x| x.to_input_value())}
            },
            MaxLength => quote! {
                maxlength={props.max_length.map(|x| x.to_string())}
            },
            Min => quote! {
                min={props.min.map(|x| x.to_input_value())}
            },
            MinLength => quote! {
                minlength={props.min_length.map(|x| x.to_string())}
            },
            Multiple => quote! {
                multiple={props.multiple}
            },
            Pattern => quote! {
                pattern={props.pattern.clone()}
            },
            Placeholder => quote! {
                placeholder={props.placeholder.clone()}
            },
            Required => quote! {
                required={props.required}
            },
            Step(_) => quote! {
                step={props.step.map(|s| s.value())}
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
            "step" => Ok(HtmlInputAttrKind::Step(Box::new(syn::Type::Verbatim(
                quote! { u32 },
            )))),
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
                    *default = Box::new(ty);
                }
            }
        }

        Ok(HtmlInputAttr { ident, kind })
    }
}
