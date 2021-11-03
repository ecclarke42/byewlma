use byewlma::{
    components::{Menu, MenuItem, MenuSection},
    layout::{columns, Column, Columns, Section},
};
use yew::prelude::*;
use yew_router::prelude::*;

pub struct App;

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

    #[at("/form/input")]
    FormInput,

    #[at("/")]
    Home,
    // TODO: not found?
}

impl Component for App {
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <Section>
                <Router<Route> render={
                    Router::render(move |route: &Route| {
                        html! {
                            <Columns>
                                <Column size={columns::Size::Narrow}>
                                    <AppMenu active_route={*route} />
                                </Column>
                                <Column>
                                    {match route {
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
                                    }}
                                </Column>
                            </Columns>
                        }
                    })
                }/>
            </Section>
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Properties)]
struct AppMenuProps {
    active_route: Route,
}

#[function_component(AppMenu)]
fn app_menu(props: &AppMenuProps) -> Html {
    html! {
        // TODO: navbar
        <Menu>
            <MenuSection label={html!("Layout")}>
                <MenuItem
                    label={html!("Columns")}
                    active={props.active_route == Route::LayoutColumns}
                    on_click={Callback::from(|_| yew_router::push_route(Route::LayoutColumns))}
                />
                <MenuItem
                    label={html!("Container")}
                    active={props.active_route == Route::LayoutContainer}
                    on_click={Callback::from(|_| yew_router::push_route(Route::LayoutContainer))}
                />
                <MenuItem
                    label={html!("Footer")}
                    active={props.active_route == Route::LayoutFooter}
                    on_click={Callback::from(|_| yew_router::push_route(Route::LayoutFooter))}
                />
                <MenuItem
                    label={html!("Hero")}
                    active={props.active_route == Route::LayoutHero}
                    on_click={Callback::from(|_| yew_router::push_route(Route::LayoutHero))}
                />
                <MenuItem
                    label={html!("Level")}
                    active={props.active_route == Route::LayoutLevel}
                    on_click={Callback::from(|_| yew_router::push_route(Route::LayoutLevel))}
                />
                <MenuItem
                    label={html!("Section")}
                    active={props.active_route == Route::LayoutSection}
                    on_click={Callback::from(|_| yew_router::push_route(Route::LayoutSection))}
                />
            </MenuSection>
            <MenuSection label={html!("Components")}>
                <MenuItem
                    label={html!("Button")}
                    active={props.active_route == Route::Button}
                    on_click={Callback::from(|_| yew_router::push_route(Route::Button))}
                />
            </MenuSection>
            <MenuSection label={html!("Form")}>
                <MenuItem
                    label={html!("Input")}
                    active={props.active_route == Route::FormInput}
                    on_click={Callback::from(|_| yew_router::push_route(Route::FormInput))}
                />
            </MenuSection>
            <MenuSection label={html!("Helpers")}>

            </MenuSection>
        </Menu>
    }
}
