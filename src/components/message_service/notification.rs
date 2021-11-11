use gloo::timers::callback::Timeout;
use yew::prelude::*;

use super::properties::{Color, Position, Size};

#[derive(Properties, Clone, PartialEq)]
pub struct NotificationProps {
    #[prop_or_default]
    pub header: Option<String>,

    pub children: Children,

    #[prop_or(true)]
    pub can_close: bool,

    #[prop_or_default]
    pub timeout: Option<std::time::Duration>,
    #[prop_or_default]
    pub on_timeout: Option<Callback<()>>,
    #[prop_or_default]
    pub on_closed: Option<Callback<()>>,

    #[prop_or(Color::Default)]
    pub color: Color,
    #[prop_or(Size::Normal)]
    pub size: Size,
    #[prop_or(Position::BottomRight)]
    pub position: Position,

    // pub margin: Option<
    /// This should be set (false) by the notification service to handle
    /// positioning of multiple notifications. By default
    #[prop_or(true)]
    pub(crate) standalone: bool,
}

/// A bulma [message](https://bulma.io/documentation/components/message/)
/// that will be displayed as a notification
pub struct Notification {
    closed: bool,
    timed_out: bool,

    timeout: Option<Timeout>,
}

pub enum Msg {
    TimedOut,
    Closed,
    Rendered,
    TimeoutAnimated,
    CloseAnimated,
    DisplayAnimated,
}

impl Component for Notification {
    type Message = Msg;
    type Properties = NotificationProps;

    fn create(ctx: &Context<Self>) -> Self {
        let timeout = {
            let link = ctx.link().clone();
            ctx.props().timeout.map(move |duration| {
                Timeout::new(duration.as_millis() as u32, move || {
                    link.send_message(Msg::TimedOut)
                })
            })
        };

        Self {
            closed: false,
            timed_out: false,
            timeout,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Rendered => false,
            Msg::TimedOut => {
                self.timed_out = true;
                self.timeout = None;
                true
            }
            Msg::TimeoutAnimated => {
                if let Some(ref on_timeout) = ctx.props().on_timeout {
                    on_timeout.emit(())
                }
                false
            }
            Msg::Closed => {
                self.closed = true;
                true
            }
            Msg::CloseAnimated => {
                if let Some(ref on_closed) = ctx.props().on_closed {
                    on_closed.emit(())
                }
                false
            }
            Msg::DisplayAnimated => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let link = ctx.link();

        // TODO: better way of putting this in the html?
        // TODO: margin property?
        // let style = if self.props.standalone {
        //     self.props.position.style("1em", "1em")
        // } else {
        //     String::new()
        // };

        // Figure out which classes to render
        let mut msg_cls = Classes::from("message byewlma-msg-svc-body");
        let mut del_cls = Classes::from("delete");
        if let Some(cls) = props.color.class() {
            msg_cls.push(cls);
        }
        if let Some(cls) = props.size.class() {
            msg_cls.push(cls);
            del_cls.push(cls);
        }
        if props.standalone {
            msg_cls.push(props.position.style());
        }

        // Animation class
        let animation_callback = if self.closed {
            msg_cls.push(props.position.animate_out_style());
            link.callback(|_| Msg::CloseAnimated)
        } else if self.timed_out {
            msg_cls.push(props.position.animate_out_style());
            link.callback(|_| Msg::TimeoutAnimated)
        } else {
            msg_cls.push(props.position.animate_in_style());
            link.callback(|_| Msg::DisplayAnimated)
        };

        // Show the header if there is a header string, or if "can-close" is true
        let header = if props.header.is_some() || props.can_close {
            let header = props.header.clone().unwrap_or_default();
            let button = if props.can_close {
                html! {<button class={del_cls} aria-label="delete" onclick={ctx.link().callback(|_| Msg::Closed)}></button>}
            } else {
                html! {}
            };
            html! {
                <div class="message-header">
                    <p>{header}</p>
                    {button}
                </div>
            }
        } else {
            html! {}
        };

        html! {
            <article class={msg_cls} onanimationend={animation_callback}>
                {header}
                <div class="message-body">
                    {props.children.clone()}
                </div>
            </article>
        }
    }
}
