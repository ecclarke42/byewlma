use byewlma::components::search_select::{
    Select, SelectDisplay, SelectFilter, SelectState, Selection,
};
use yew::prelude::*;

fn main() {
    yew::start_app::<App>();
}

/// Test data struct
#[derive(Clone, PartialEq)]
pub struct Data {
    name: String,
    value: u32,
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Simple yew application that spawns a notification
pub struct App {
    select_display: SelectDisplay<Data>,

    a_data: SelectState<Data, Vec<Data>>,
    b_data: SelectState<Data, Vec<Data>>,
    c_data: SelectState<Data, Vec<Data>>,
}

pub enum Msg {
    SelectedA(usize),
    ClearedA(usize),

    SelectedB(usize),
    ClearedB(usize),

    SelectedC(usize),
    ClearedC(usize),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let test_data = vec![
            Data {
                name: String::from("First"),
                value: 0,
            },
            Data {
                name: String::from("Second"),
                value: 1,
            },
            Data {
                name: String::from("Third"),
                value: 2,
            },
            Data {
                name: String::from("Fourth"),
                value: 3,
            },
            Data {
                name: String::from("Fifth"),
                value: 4,
            },
            Data {
                name: String::from("Something else with \"first\""),
                value: 5,
            },
        ];

        let filter = SelectFilter::new(|item: &Data, search: &str| -> bool {
            item.name.to_lowercase().contains(&search.to_lowercase())
        });

        Self {
            select_display: SelectDisplay::new(|item: &Data| item.to_string()),
            a_data: SelectState::new(test_data.clone(), Selection::one(0), filter.clone()),
            b_data: SelectState::new(test_data.clone(), Selection::none(), filter.clone()),
            c_data: SelectState::new(test_data, Selection::empty(), filter),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SelectedA(index) => self.a_data.select(index),
            Msg::ClearedA(index) => self.a_data.deselect(index),

            Msg::SelectedB(index) => self.b_data.select(index),
            Msg::ClearedB(index) => self.b_data.deselect(index),

            Msg::SelectedC(index) => self.c_data.select(index),
            Msg::ClearedC(index) => self.c_data.deselect(index),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <main>
                <div class="field">
                    <label class="label">{"Select Single, Non-Nullable Field"}</label>
                    <div class="control">
                        <Select<Data, Vec<Data>>
                            state={self.a_data.clone()}
                            display={self.select_display.clone()}
                            onselected={link.callback(Msg::SelectedA)}
                        />
                    </div>
                </div>
                <div class="field">
                    <label class="label">{"Select Single, Nullable Field"}</label>
                    <div class="control">
                        <Select<Data, Vec<Data>>
                            state={self.b_data.clone()}
                            display={self.select_display.clone()}
                            onselected={link.callback(Msg::SelectedB)}
                        />
                    </div>
                </div>
                <div class="field">
                    <label class="label">{"Select Multiple Fields"}</label>
                    <div class="control">
                        <Select<Data, Vec<Data>>
                            state={self.c_data.clone()}
                            display={self.select_display.clone()}
                            onselected={link.callback(Msg::SelectedC)}
                            onremoved={link.callback(Msg::ClearedC)}
                        />
                    </div>
                </div>
                <div class="field">
                    <label class="label">{"Select Multiple Fields (Clone, omit selections from options)"}</label>
                    <div class="control">
                        <Select<Data, Vec<Data>>
                            omit_selected={true}
                            state={self.c_data.clone()}
                            display={self.select_display.clone()}
                            onselected={link.callback(Msg::SelectedC)}
                            onremoved={link.callback(Msg::ClearedC)}
                        />
                    </div>
                </div>
                <div class="field">
                    <label class="label">{"Dropdown for comparison"}</label>
                    <div class="control">
                        <div class="dropdown">
                            <div class="dropdown-trigger">
                                <button class="button">
                                    <span>{"Dropdown"}</span>
                                    <span class="icon is-small">
                                        <i class="fas fa-angle-down" />
                                    </span>
                                </button>
                            </div>
                            <div class="dropdown-menu">
                            </div>
                        </div>
                    </div>
                </div>
            </main>
        }
    }
}
