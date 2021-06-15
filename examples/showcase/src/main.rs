#[macro_use]
mod util;

mod app;

fn main() {
    yew::start_app::<app::App>();
}

mod routes {
    pub mod button;
}
