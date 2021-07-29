#![recursion_limit = "256"]

mod html_input;

#[proc_macro_attribute]
pub fn html_input(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    html_input::transform_stream(attr, item)
}
