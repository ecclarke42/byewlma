use quote::quote;
use syn::parse_macro_input;

const BULMA_CLASS_ATTR: &str = "bulma_class";

pub fn transform_stream(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as syn::DeriveInput);

    let type_name = input.ident;
    let variants = if let syn::Data::Enum(data) = input.data {
        data.variants
            .iter()
            .filter_map(|v| {
                class_name_attr(&v.attrs).map(|lit| {
                    let ident = &v.ident;
                    match &v.fields {
                        syn::Fields::Unit => quote! { Self::#ident => #lit, },
                        syn::Fields::Named(_) => quote! { Self::#ident {..} => #lit, },
                        syn::Fields::Unnamed(_) => quote! { Self::#ident(_) => #lit, },
                    }
                })
            })
            .collect::<Vec<_>>()
    } else {
        panic!("Only enums are supported")
    };

    proc_macro::TokenStream::from(quote! {
        impl crate::util::class::BulmaClass for #type_name {
            fn class(&self) -> &'static str {
                match self {
                    #(#variants)*
                }
            }
        }

        impl From<#type_name> for yew::Classes {
            fn from(bulma: #type_name) -> Self {
                let mut classes = Self::new();
                unsafe {
                    classes.unchecked_push(bulma.class());
                }
                classes
            }
        }

        // impl<T: crate::util::class::BulmaClass> From<&T> for yew::Classes {
        //     fn from(bulma: &T) -> Self {
        //         bulma.to_yew()
        //     }
        // }
        // impl<T: crate::util::class::BulmaClass> Into<yew::Classes> for T {
        //     fn into(self) -> yew::Classes {
        //         self.to_yew()
        //     }
        // }
    })
}

fn class_name_attr(attrs: &[syn::Attribute]) -> Option<syn::LitStr> {
    let mut literal = None;
    for attr in attrs {
        if let syn::Meta::NameValue(syn::MetaNameValue {
            path,
            lit: syn::Lit::Str(lit_str),
            ..
        }) = attr.parse_meta().expect("Failed to parse attribute")
        {
            if path.is_ident(BULMA_CLASS_ATTR) {
                if literal.is_some() {
                    panic!("Bulma class cannot be specified twice on a variant");
                }
                if lit_str.value().contains(' ') {
                    panic!("Bulma class cannot contain spaces")
                }
                literal = Some(lit_str);
            }
        }
    }
    literal
}
