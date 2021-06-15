use byewlma::prelude::*;
use yew::prelude::*;

#[derive(Debug)]
pub struct FormInputShowcase {
    link: ComponentLink<Self>,
}

pub enum FormInputShowcaseMsg {}

impl Component for FormInputShowcase {
    type Properties = ();
    type Message = FormInputShowcaseMsg;
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // match msg {}
        false
    }
    fn view(&self) -> Html {
        html! {
            <>
                <Hero color={SemanticColor::Primary}>
                    <Title>{"Form Inputs"}</Title>
                    // <Subtitle>{"The bulma"}</Title>
                </Hero>

                <Section>
                    <Subtitle>{"Input Types"}</Subtitle>
                    <Field label={"Text"}>
                        <TextInput />
                    </Field>
                    // <Columns>
                    //     <Column size={columns::Size::OneThird}><SimpleButton/></Column>
                    //     <Column ref={self.simple_button_code_ref.clone()} />
                    // </Columns>
                </Section>
            </>
        }
    }
}
