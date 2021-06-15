mod helpers;

pub use helpers::*;

#[macro_use]
mod util {
    #[macro_use]
    pub mod pure_wrapper;
}

pub mod components {
    pub mod prelude {
        pub use yew::prelude::*;
        pub use yewtil::{NeqAssign, Pure, PureComponent};

        pub use crate::helpers::{Color, Size};
    }

    mod icon_kind;

    pub mod block;
    pub mod r#box;
    // TODO: pub mod breadcrumb\
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
    pub mod title;

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
    pub use title::{Subtitle, Title};
}

pub mod form {
    pub mod field;
    pub mod input;
}

pub mod layout {
    pub mod columns;
    pub mod container;
    pub mod footer;
    pub mod hero;
    // TODO: pub mod level;
    // TODO: pub mod media_object;
    pub mod section;
    // TODO: pub mod tiles;

    pub use columns::{Column, Columns};
    pub use hero::Hero;
    pub use section::Section;
}

pub mod typography;

pub mod prelude {
    pub use crate::components::*;
    pub use crate::form::*;
    pub use crate::helpers::*;
    pub use crate::layout::*;
    pub use crate::typography::*;
}
