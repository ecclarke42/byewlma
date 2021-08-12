use byewlma::{prelude::*, components::{Button, Title}};
use yew::{prelude::*, web_sys::Element};
use yewtil::future::LinkFuture;

#[derive(Debug)]
pub struct ButtonShowcase {
    link: ComponentLink<Self>,

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
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_future(async {
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
            link,
            simple_button_code_ref: NodeRef::default(),
            simple_button_code_content: None,
        }
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            // just force re-render
            ButtonShowcaseMsg::LoadedSimpleButtonContent(content) => {
                self.simple_button_code_content = Some(content);
                true
            }
        }
    }
    fn view(&self) -> Html {
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
    fn rendered(&mut self, _first_render: bool) {
        if let Some(content) = &self.simple_button_code_content {
            let elem = self
                .simple_button_code_ref
                .cast::<Element>()
                .expect("failed to cast node ref to element");
            elem.set_inner_html(content.as_str());
        }
    }
}

const SIMPLE_BUTTON_CONTENT_START: u32 = line!();
pub struct SimpleButton {
    link: ComponentLink<Self>,
    clicks: u32,
}
pub enum SimpleButtonMsg {
    Clicked,
}
impl Component for SimpleButton {
    type Properties = ();
    type Message = SimpleButtonMsg;
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, clicks: 0 }
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            SimpleButtonMsg::Clicked => {
                self.clicks += 1;
                true
            }
        }
    }
    fn view(&self) -> Html {
        html! {
            <Button on_click={self.link.callback(|_| SimpleButtonMsg::Clicked)}>
                {"Clicked "}
                {self.clicks}
                {" times!"}
            </Button>
        }
    }
}
const SIMPLE_BUTTON_CONTENT_LINES: u32 = line!() - SIMPLE_BUTTON_CONTENT_START - 1;
