/// Extension trait for yew::Classes
pub(crate) trait AddClass {
    fn add<T: BulmaClass>(&mut self, class: &T);
}

// Bulma classes should all be without spaces, so we'll use the
// unsafe push here
impl AddClass for yew::Classes {
    fn add<T: BulmaClass>(&mut self, class: &T) {
        unsafe { self.unchecked_push(class.class()) }
    }
}

pub trait BulmaClass {
    /// Class should only produce a valid class name (i.e. a value without
    /// spaces!)
    fn class(&self) -> &'static str;

    fn to_yew(&self) -> yew::Classes {
        yew::Classes::from(self.class())
    }
}
// impl<T: BulmaClass> AsRef<str> for T {
//     fn as_ref(&self) -> &str {
//         self.class()
//     }
// }
// impl<T: BulmaClass> Into<&'static str> for T {
//     fn into(self) -> &'static str {
//         self.class()
//     }
// }

// impl<T: BulmaClass> From<&T> for yew::Classes {
//     fn from(bulma: &T) -> Self {
//         bulma.to_yew()
//     }
// }
// impl<T: BulmaClass> Into<yew::Classes> for T {
//     fn into(self) -> yew::Classes {
//         self.to_yew()
//     }
// }
