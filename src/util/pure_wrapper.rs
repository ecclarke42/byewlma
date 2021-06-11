use yew::{Children, Classes, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct PureWrapper<T: Properties> {
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub(crate) children: Children,

    inner: T,
}
