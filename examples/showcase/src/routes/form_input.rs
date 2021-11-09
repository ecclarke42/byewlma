use byewlma::{
    components::{Subtitle, Title},
    form::{Control, Field, PasswordInput, TextInput},
    prelude::*,
};
use yew::prelude::*;

#[function_component(FormInputShowcase)]
pub fn form_input_showcase(_props: &()) -> Html {
    html! {
        <>
            <Hero color={SemanticColor::Primary}>
                <Title>{"Form Inputs"}</Title>
                // <Subtitle>{"The bulma"}</Title>
            </Hero>

            <Section>
                <Subtitle>{"Input Types"}</Subtitle>
                <Field label={{html!{"Text"}}}>
                    <Control>
                        <TextInput />
                    </Control>
                </Field>
                    <Field label={{html!{"Password"}}}>
                    <Control>
                        <PasswordInput />
                    </Control>
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
