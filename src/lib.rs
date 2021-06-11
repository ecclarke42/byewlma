mod helpers;

pub use helpers::*;

macro_rules! pub_use {
    ($mod:ident::{$($item:ident),*}) => {
        mod $mod;
        pub use $mod::{$($item),*};
    };
}

mod util {
    pub mod pure_wrapper;
}

pub mod components {
    pub(crate) mod prelude {
        pub use yew::prelude::*;
        pub use yewtil::{NeqAssign, Pure, PureComponent};

        pub use crate::helpers::{Color, Size};
        pub use crate::util::pure_wrapper::PureWrapper;
    }

    // pub_use!(button::{Button});
    // pub_use!(icon::{Icon});

    mod icon_kind;

    pub mod block;
    // TODO: pub mod breadcrumb
    pub mod r#box;
    pub mod button;
    pub mod buttons;
    // TODO: pub mod card;
    pub mod content;
    // TODO: pub mod dropdown;
    pub mod image;
    // TODO: pub mod delete;
    pub mod icon;
    pub mod menu;
    // TODO: pub mod message;
    // TODO: pub mod modal;
    // TODO: pub mod navbar;
    pub mod notification;
    // TODO: pub mod pagination;
    // TODO: pub mod panel;
    pub mod progress;
    // TODO: pub mod table;
    // TODO: pub mod tabs;
    // TODO: pub mod tag;
    // TODO: pub mod title

    pub use block::Block;
    pub use button::Button;
    pub use buttons::Buttons;
    pub use content::Content;
    pub use icon::Icon;
    pub use image::Image;
    pub use menu::{Menu, MenuItem, MenuSection};
    pub use notification::Notification;
    pub use progress::Progress;
    pub use r#box::Box;
}

pub mod form {
    pub mod field;
    // pub mod input;
}

pub mod layout {
    pub mod columns;
}
