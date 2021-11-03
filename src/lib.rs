#[macro_use]
mod util {

    #[macro_use]
    pub mod class;

    #[macro_use]
    pub mod pure_wrapper;
}

mod innerlude {
    pub(crate) use std::borrow::Cow;

    pub(crate) use byewlma_macros::BulmaClass;
    pub(crate) use yew::prelude::*;

    pub(crate) use crate::util::class::{AddClass, BulmaClass};
}

mod helpers;
pub use helpers::*;

pub mod components {
    mod icon_kind;

    pub mod block;
    pub mod r#box;
    // TODO: pub mod breadcrumb;
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

    // More complex components
    pub mod message_service;
    pub mod search_select;

    // Re-export main components
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

pub mod form;

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
    pub use container::{Container, ContainerWidth};
    pub use hero::Hero;
    pub use section::Section;
}

// TODO: just traits in prelude?
pub mod prelude {
    // pub use crate::components::*;
    // pub use crate::form::*; // InputValue
    pub use crate::{helpers::*, layout::*};
}

// Re-export Yew
pub use yew;
