use std::borrow::Cow;

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
    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
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
                    <Field label={Some(Cow::<'static, str>::from("Text"))}>
                        <TextInput />
                    </Field>
                    <Field label={Some(Cow::<'static, str>::from("Password"))}>
                        <PasswordInput />
                    </Field>
                    // <Field label={"Color"}>
                    //     <Input<input::types::Color> />
                    // </Field>
                    // <Field label={"Date"}>
                    //     <Input<input::types::Date> />
                    // </Field>
                    // <Field label={"DateTime"}>
                    //     <Input<input::types::DateTime> />
                    // </Field>
                    // <Field label={"Email"}>
                    //     <Input<input::types::Email> />
                    // </Field>
                    // <Field label={"Hidden"}>
                    //     <Input<input::types::Hidden> />
                    // </Field>
                    // <Field label={"Month"}>
                    //     <Input<input::types::Month> />
                    // </Field>
                    // <Field label={"Integer"}>
                    //     <Input<input::types::Integer> />
                    // </Field>
                    // <Field label={"Float"}>
                    //     <Input<input::types::Float> />
                    // </Field>
                    // <Field label={"Range"}>
                    //     // <Input<input::types::Range> />
                    // </Field>
                    // <Field label={"Phone"}>
                    //     <Input<input::types::Phone> />
                    // </Field>
                    // <Field label={"Time"}>
                    //     <Input<input::types::Time> />
                    // </Field>
                    // <Field label={"Week"}>
                    //     <Input<input::types::Week> />
                    // </Field>

                    // <Columns>
                    //     <Column size={columns::Size::OneThird}><SimpleButton/></Column>
                    //     <Column ref={self.simple_button_code_ref.clone()} />
                    // </Columns>
                </Section>
            </>
        }
    }
}
