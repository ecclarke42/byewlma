// mod color;
// mod size;

// pub use color::Color;

/// Return the classes for some type of attribute
pub trait Attribute {
    fn as_classes(&self) -> yew::html::Classes;
}

impl<T: Copy + Into<yew::html::Classes>> Attribute for T {
    fn as_classes(&self) -> yew::html::Classes {
        (*self).into()
    }
}
