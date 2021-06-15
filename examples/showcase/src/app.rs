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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Routable)]
pub enum Route {
    #[at("/layout/columns")]
    LayoutColumns,
    #[at("/layout/container")]
    LayoutContainer,
    #[at("/layout/footer")]
    LayoutFooter,
    #[at("/layout/hero")]
    LayoutHero,
    #[at("/layout/level")]
    LayoutLevel,
    #[at("/layout/section")]
    LayoutSection,

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
            <Section>
                <Router<Route> render={
                    Router::render(move |route: &Route| {
                        html! {
                            <Columns>
                                <Column size={columns::Size::Narrow}>{ Self::render_menu(link.clone(), route) }</Column>
                                <Column>
                                    { Self::render_route(route) }
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

            Route::Button => html! { <crate::routes::button::ButtonShowcase /> },
        }
    }
}
