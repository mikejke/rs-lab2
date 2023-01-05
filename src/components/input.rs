use iced::widget::{Column, Text, TextInput};
use iced::{Element, Length, Subscription};

#[allow(unused)]
#[derive(Debug)]
pub struct MyInput {
    value: String,
    label: String,
    with_error: bool,
}

#[derive(Debug, Clone)]
pub enum MyInputMessage {
    InputChanged(String),
}

impl MyInput {
    pub fn new(value: &str, label: &str) -> Self {
        Self {
            label: label.to_owned(),
            value: value.to_owned(),
            with_error: false,
        }
    }

    pub fn update(&mut self, message: MyInputMessage) {
        match message {
            MyInputMessage::InputChanged(value) => self.value = value,
        }
    }

    pub fn view(&self) -> Element<MyInputMessage> {
        Column::new()
            .push(Text::new(&self.label).width(Length::Fill))
            .push(
                TextInput::new(&self.label, &self.value, MyInputMessage::InputChanged)
                    .width(Length::Fill),
            )
            .into()
    }

    pub fn subscription(&self) -> Subscription<MyInputMessage> {
        Subscription::none()
    }
}
