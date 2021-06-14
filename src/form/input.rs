use crate::components::prelude::*;

#[derive(Debug, Default, PartialEq, Clone, Properties)]
pub struct PureTextInput {
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub label: Option<String>,

    #[prop_or_default]
    pub help: Option<String>,
}

impl PureComponent for PureTextInput {
    fn render(&self) -> Html {
        let mut class = self.class.clone();
        class.push("input");

        let label = if let Some(label) = &self.label {
            html! { <label class="label">{label.clone()}</label> }
        } else {
            html! {}
        };

        let help = if let Some(help) = &self.help {
            html! { <p class="help">{help.clone()}</p> }
        } else {
            html! {}
        };

        html! {
            <div class="control">
                <input type="" />
            </div>
        }
    }
}

// #[derive(Debug, PartialEq, Clone, Copy)]
// pub enum InputType {
//     Button,
//     Checkbox,
//     Color,
//     Date,
//     DateTime,
//     Email,
//     File,
//     Hidden,
//     Image,
//     Month,
//     Number,
//     Password,
//     Radio,
//     Range,
//     Reset,
//     Search,
//     Submit,
//     Telephone,
//     Text,
//     Time,
//     Url,
//     Week,
// }

// impl Default for InputType {
//     fn default() -> Self {
//         InputType::Text
//     }
// }

// impl InputType {
//     pub fn name(&self) -> &'static str {
//         use InputType::*;
//         match self {
//             Button => "button",
//             Checkbox => "checkbox",
//             Color => "color",
//             Date => "date",
//             DateTime => "datetime-local",
//             Email => "email",
//             File => "file",
//             Hidden => "hidden",
//             Image => "image",
//             Month => "month",
//             Number => "number",
//             Password => "password",
//             Radio => "radio",
//             Range => "range",
//             Reset => "reset",
//             Search => "search",
//             Submit => "submit",
//             Telephone => "tel",
//             Text => "text",
//             Time => "time",
//             Url => "url",
//             Week => "week",
//         }
//     }
// }
