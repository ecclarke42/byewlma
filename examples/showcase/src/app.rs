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

type RouteAnchor = yew_router::components::Link<Route>;

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
                    is_active={props.active_route == Route::LayoutColumns}
                    label={html!(<RouteAnchor route={Route::LayoutColumns}>{"Columns"}</RouteAnchor>)}
                />
                <MenuItem
                    is_active={props.active_route == Route::LayoutContainer}
                    label={html!(<RouteAnchor route={Route::LayoutContainer}>{"Container"}</RouteAnchor>)}
                />
                <MenuItem
                    is_active={props.active_route == Route::LayoutFooter}
                    label={html!(<RouteAnchor route={Route::LayoutFooter}>{"Footer"}</RouteAnchor>)}
                />
                <MenuItem
                    is_active={props.active_route == Route::LayoutHero}
                    label={html!(<RouteAnchor route={Route::LayoutHero}>{"Hero"}</RouteAnchor>)}
                />
                <MenuItem
                    is_active={props.active_route == Route::LayoutLevel}
                    label={html!(<RouteAnchor route={Route::LayoutLevel}>{"Level"}</RouteAnchor>)}
                />
                <MenuItem
                    is_active={props.active_route == Route::LayoutSection}
                    label={html!(<RouteAnchor route={Route::LayoutSection}>{"Section"}</RouteAnchor>)}
                />
            </MenuSection>
            <MenuSection label={html!("Components")}>
                <MenuItem
                    is_active={props.active_route == Route::Button}
                    label={html!(<RouteAnchor route={Route::Button}>{"Button"}</RouteAnchor>)}
                />
            </MenuSection>
            <MenuSection label={html!("Form")}>
                <MenuItem
                    is_active={props.active_route == Route::FormInput}
                    label={html!(<RouteAnchor route={Route::FormInput}>{"Input"}</RouteAnchor>)}
                />
            </MenuSection>
            <MenuSection label={html!("Helpers")}>

            </MenuSection>
        </Menu>
    }
}
