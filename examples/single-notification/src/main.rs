use byewlma::components::message_service::Notification;
use yew::prelude::*;

fn main() {
    yew::start_app::<App>();
}

/// Simple yew application that spawns a notification
pub struct App {
    notify: bool,
}

pub enum Msg {
    OpenNotification,
    CloseNotification,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { notify: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OpenNotification => {
                self.notify = true;
                true
            }
            Msg::CloseNotification => {
                self.notify = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let open = ctx.link().callback(|_| Msg::OpenNotification);
        let close = ctx.link().callback(|_| Msg::CloseNotification);
        html! {
            <main class="byewlma-msg-svc-parent">
                <h1>{"Single Notification Example"}</h1>
                <button onclick={open}>{"Notify Me"}</button>

                {if self.notify {
                    html! {
                        <Notification
                            header={"a message".to_string()}
                            timeout={std::time::Duration::from_secs(5)}
                            on_timeout={close.clone()}
                            on_closed={close.clone()}
                        >
                            <p>{"body"}</p>
                            <p>{"body"}</p>
                            <p>{"body"}</p>
                        </Notification>
                    }
                } else {
                    html!{}
                }}
            </main>
        }
    }
}
