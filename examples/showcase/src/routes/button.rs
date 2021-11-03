use byewlma::{
    components::{Button, Title},
    prelude::*,
};
use yew::prelude::*;

#[derive(Debug)]
pub struct ButtonShowcase {
    simple_button_code_ref: NodeRef,
    simple_button_code_content: Option<String>,
}

pub enum ButtonShowcaseMsg {
    LoadedSimpleButtonContent(String),
}

const FILE_SOURCE: &str = include_str!("button.rs");

impl Component for ButtonShowcase {
    type Properties = ();
    type Message = ButtonShowcaseMsg;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            let simple_button_content = FILE_SOURCE
                .lines()
                .skip(SIMPLE_BUTTON_CONTENT_START as usize)
                .take(SIMPLE_BUTTON_CONTENT_LINES as usize)
                .collect::<Vec<_>>()
                .join("\n");
            let syntaxed_simple_button_content =
                crate::util::hightlight_snippet(&simple_button_content);
            ButtonShowcaseMsg::LoadedSimpleButtonContent(syntaxed_simple_button_content)
        });
        Self {
            simple_button_code_ref: NodeRef::default(),
            simple_button_code_content: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // just force re-render
            ButtonShowcaseMsg::LoadedSimpleButtonContent(content) => {
                self.simple_button_code_content = Some(content);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Hero color={SemanticColor::Primary}>
                    <Title>{"Button"}</Title>
                    // <Subtitle>{"The bulma"}</Title>
                </Hero>

                <Section>
                    <Columns>
                        <Column size={columns::Size::OneThird}><SimpleButton/></Column>
                        <Column ref={self.simple_button_code_ref.clone()} />
                    </Columns>
                </Section>
            </>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        if let Some(content) = &self.simple_button_code_content {
            let elem = self
                .simple_button_code_ref
                .cast::<web_sys::Element>()
                .expect("failed to cast node ref to element");
            elem.set_inner_html(content.as_str());
        }
    }
}

const SIMPLE_BUTTON_CONTENT_START: u32 = line!();

#[function_component(SimpleButton)]
pub fn simple_button(_props: &()) -> Html {
    let clicks = use_state(|| 0u32);

    let on_click = {
        let clicks = clicks.clone();
        Callback::from(move |_| clicks.set(*clicks + 1))
    };

    html! {
        <Button {on_click}>
            {"Clicked "}
            { *clicks }
            {" times!"}
        </Button>
    }
}

const SIMPLE_BUTTON_CONTENT_LINES: u32 = line!() - SIMPLE_BUTTON_CONTENT_START - 1;
