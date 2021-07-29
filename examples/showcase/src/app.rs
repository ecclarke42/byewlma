use byewlma::{
    components::{Menu, MenuItem, MenuSection},
    layout::{
        Section, {columns, Column, Columns},
    },
};
use yew::prelude::*;
use yew_router::prelude::*;

pub struct App {
    link: ComponentLink<Self>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Switch)]
pub enum Route {
    #[to = "/layout/columns"]
    LayoutColumns,
    #[to = "/layout/container"]
    LayoutContainer,
    #[to = "/layout/footer"]
    LayoutFooter,
    #[to = "/layout/hero"]
    LayoutHero,
    #[to = "/layout/level"]
    LayoutLevel,
    #[to = "/layout/section"]
    LayoutSection,

    #[to = "/components/button"]
    Button,

    #[to = "/form/input"]
    FormInput,

    #[to = "/"]
    Home,
}

pub enum Msg {
    Route(Route),
}

impl Component for App {
    type Properties = ();
    type Message = Msg;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        App { link }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Route(route) => {
                // TODO?
                // yew_router::push_route(self);
                let mut r = String::new();
                let state = wasm_bindgen::JsValue::NULL;
                route.build_route_section::<()>(&mut r);
                yew::utils::window()
                    .history()
                    .expect("failed to access history")
                    .push_state_with_url(&state, "", Some(r.as_str()));
                true
            }
        }
    }

    fn view(&self) -> Html {
        let link = self.link.clone();
        html! {
            <Section>
                <Router<Route> render={
                    Router::render(move |route: Route| {
                        html! {
                            <Columns>
                                <Column size={columns::Size::Narrow}>{ Self::render_menu(link.clone(), &route) }</Column>
                                <Column>
                                    { Self::render_route(&route) }
                                </Column>
                            </Columns>
                        }
                    })
                }/>
            </Section>
        }
    }
}

impl App {
    pub fn render_menu(link: ComponentLink<Self>, route: &Route) -> Html {
        html! {
            // TODO: navbar
            <Menu>
                <MenuSection label={html!{"Layout"}}>
                    <MenuItem
                        label={html!{"Columns"}}
                        active={route == &Route::LayoutColumns}
                        on_click={link.callback(|_| Msg::Route(Route::LayoutColumns))}
                    />
                    <MenuItem
                        label={html!{"Container"}}
                        active={route == &Route::LayoutContainer}
                        on_click={link.callback(|_| Msg::Route(Route::LayoutContainer))}
                    />
                    <MenuItem
                        label={html!{"Footer"}}
                        active={route == &Route::LayoutFooter}
                        on_click={link.callback(|_| Msg::Route(Route::LayoutFooter))}
                    />
                    <MenuItem
                        label={html!{"Hero"}}
                        active={route == &Route::LayoutHero}
                        on_click={link.callback(|_| Msg::Route(Route::LayoutHero))}
                    />
                    <MenuItem
                        label={html!{"Level"}}
                        active={route == &Route::LayoutLevel}
                        on_click={link.callback(|_| Msg::Route(Route::LayoutLevel))}
                    />
                    <MenuItem
                        label={html!{"Section"}}
                        active={route == &Route::LayoutSection}
                        on_click={link.callback(|_| Msg::Route(Route::LayoutSection))}
                    />
                </MenuSection>
                <MenuSection label={html!{"Components"}}>
                    <MenuItem
                        label={html!{"Button"}}
                        active={route == &Route::Button}
                        on_click={link.callback(|_| Msg::Route(Route::Button))}
                    />
                </MenuSection>
                <MenuSection label={html!{"Form"}}>
                    <MenuItem
                        label={html!{"Input"}}
                        active={route == &Route::FormInput}
                        on_click={link.callback(|_| Msg::Route(Route::FormInput))}
                    />
                </MenuSection>
                <MenuSection label={html!{"Helpers"}}>

                </MenuSection>
            </Menu>
        }
    }

    pub fn render_route(route: &Route) -> Html {
        match route {
            Route::Home => html! {"home"},

            Route::LayoutColumns => html! { "columns" },
            Route::LayoutContainer => html! { "container" },
            Route::LayoutFooter => html! { "footer" },
            Route::LayoutHero => html! { "hero" },
            Route::LayoutLevel => html! { "level" },
            Route::LayoutSection => html! { "section" },

            // Route::Button => html! { <crate::routes::button::ButtonShowcase /> },
            Route::Button => html! { "Button Showcase" }, // Syntax highlighter is reeaaaaally slow

            Route::FormInput => html! { <crate::routes::form_input::FormInputShowcase /> },
        }
    }
}
