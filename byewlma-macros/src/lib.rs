#![recursion_limit = "256"]

mod bulma_class;
mod html_input;

#[proc_macro_attribute]
pub fn html_input(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    html_input::transform_stream(attr, item)
}

#[proc_macro_derive(BulmaClass, attributes(bulma_class))]
pub fn bulma_class(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    bulma_class::transform_stream(item)
}
