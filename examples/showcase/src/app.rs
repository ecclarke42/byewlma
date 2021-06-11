use byewlma::components::{Menu, MenuItem, MenuSection};
use yew::prelude::*;
use yew_router::prelude::*;

pub struct App {
    link: ComponentLink<Self>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Routable)]
pub enum Route {
    #[at("/components/button")]
    Button,

    #[at("/")]
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
                yew_router::push_route(route);
                true
            }
        }
    }

    fn view(&self) -> Html {
        let link = self.link.clone();
        html! {
            <main>
                <Router<Route> render={
                    Router::render(move |route: &Route| {
                        html! {
                            <>
                                { Self::render_menu(link.clone(), route) }
                                { Self::render_route(route) }
                            </>
                        }
                    })
                }/>
            </main>
        }
    }
}

impl App {
    pub fn render_menu(link: ComponentLink<Self>, route: &Route) -> Html {
        html! {
            // TODO: navbar
            <Menu>
                <MenuSection label={html!{"Components"}}>
                    <MenuItem
                        label={html!{"Button"}}
                        active={route == &Route::Button}
                        on_click={link.callback(|_| Msg::Route(Route::Button))}
                    />
                </MenuSection>
            </Menu>
        }
    }

    pub fn render_route(route: &Route) -> Html {
        match route {
            Route::Home => html! {"home"},

            Route::Button => html! { <crate::routes::button::Button /> },
        }
    }
}
