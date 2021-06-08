mod attributes;

pub use attributes::*;

pub mod components {
    mod prelude {
        pub use yew::prelude::*;
    }

    pub mod button;
}
