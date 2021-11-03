// // TODO: use these in more places? crate-specific pure?

// macro_rules! pure_props {
//     {
//         $(#[$meta:meta])*
//         pub struct $name:ident {
//             $(
//                 $(#[$attrs:meta])*
//                 $vis:vis $field:ident: $field_ty:ty,
//             )*
//         }
//     } => {
//         paste::paste! {
//             $(#[$meta])*
//             pub type $name = Pure<[<Pure $name>]>;

//             #[derive(Debug, Default, PartialEq, Clone, Properties)]
//             pub struct [<Pure $name>] {
//                 #[prop_or_default]
//                 pub id: Option<Cow<'static, str>>,

//                 #[prop_or_default]
//                 pub class: Classes,

//                 #[prop_or_default]
//                 pub style: Option<Cow<'static, str>>,

//                 #[prop_or_default]
//                 pub children: Children,

//                 $(
//                     $(#[$attrs])*
//                     $vis $field: $field_ty
//                 ),*
//             }
//         }
//     };
// }

// // TODO: something other than div?
// macro_rules! pure_wrapper {
//     ($name:ident: <$tag:ident class=$class:literal/>) => {
//         paste::paste! {
//             impl PureComponent for [<Pure $name>] {
//                 fn render(&self) -> Html {
//                     let mut class = self.class.clone();
//                     class.push($class);
//                     html! {
//                         <$tag id={self.id.clone()} class={class} style={self.style.clone()}>
//                             {for self.children.iter()}
//                         </$tag>
//                     }
//                 }
//             }
//         }
//     };
// }
