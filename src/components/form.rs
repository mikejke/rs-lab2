use crate::components::MyInput;
use iced::{
    widget::{Button, Column},
    Alignment, Element, Length, Subscription,
};

use super::MyInputMessage;

#[derive(Debug, Clone, Copy)]
pub struct MyFormData {
    pub begin: f32,
    pub end: f32,
    pub y0: f32,
    pub count: usize,
}

#[derive(Debug)]
pub struct MyForm {
    data: MyFormData,
    begin_input: MyInput,
    end_input: MyInput,
    count_input: MyInput,
    y0_input: MyInput,
}

#[derive(Debug, Clone)]
pub enum MyFormMessage {
    SubmitEvent(MyFormData),
    ResetEvent,
    BeginChanged(MyInputMessage),
    EndChanged(MyInputMessage),
    CountChanged(MyInputMessage),
    Y0Changed(MyInputMessage),
}

impl MyForm {
    pub fn new(initial: (f32, f32, f32, usize)) -> Self {
        let (begin, end, y0, count) = initial;
        Self {
            data: MyFormData {
                begin,
                end,
                y0,
                count,
            },
            begin_input: MyInput::new(&begin.to_string(), "Begin"),
            end_input: MyInput::new(&end.to_string(), "End"),
            count_input: MyInput::new(&count.to_string(), "Count"),
            y0_input: MyInput::new(&y0.to_string(), "Y0"),
        }
    }

    pub fn update(&mut self, message: MyFormMessage) {
        match message {
            MyFormMessage::SubmitEvent(data) => self.data = data,
            MyFormMessage::ResetEvent => {
                self.data = MyFormData {
                    begin: 0.0,
                    end: 0.0,
                    y0: 0.0,
                    count: 0,
                };
                println!("Reset");
                self.begin_input.update(MyInputMessage::InputChanged(self.data.begin.to_string()));
                self.end_input.update(MyInputMessage::InputChanged(self.data.end.to_string()));
                self.count_input.update(MyInputMessage::InputChanged(self.data.count.to_string()));
                self.y0_input.update(MyInputMessage::InputChanged(self.data.y0.to_string()));
            }
            MyFormMessage::BeginChanged(msg) => {
                match msg {
                    MyInputMessage::InputChanged(value) => {
                        self.data.begin = value.parse().unwrap_or(0.0);
                        self.begin_input.update(MyInputMessage::InputChanged(self.data.begin.to_string()))
                    }
                }
            },
            MyFormMessage::EndChanged(msg) => {
                match msg {
                    MyInputMessage::InputChanged(value) => {
                        self.data.end = value.parse().unwrap_or(0.0);
                        self.end_input.update(MyInputMessage::InputChanged(self.data.end.to_string()))
                    }
                }
            },
            MyFormMessage::CountChanged(msg) => {
                match msg {
                    MyInputMessage::InputChanged(value) => {
                        self.data.count = value.parse().unwrap_or(0);
                        self.count_input.update(MyInputMessage::InputChanged(self.data.count.to_string()))
                    }
                }
            },
            MyFormMessage::Y0Changed(msg) => {
                match msg {
                    MyInputMessage::InputChanged(value) => {
                        self.data.y0 = value.parse().unwrap_or(0.0);
                        self.y0_input.update(MyInputMessage::InputChanged(self.data.y0.to_string()))
                    }
                }
            },
        }
    }

    pub fn view(&self) -> Element<MyFormMessage> {
        let button = Button::new("Submit")
            .on_press(MyFormMessage::SubmitEvent(self.data.clone()));

        let reset_btn = Button::new("Reset")
            .on_press(MyFormMessage::ResetEvent);

        Column::new()
            .spacing(10)
            .align_items(Alignment::Start)
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .push(self.begin_input.view().map(MyFormMessage::BeginChanged))
            .push(self.end_input.view().map(MyFormMessage::EndChanged))
            .push(self.count_input.view().map(MyFormMessage::CountChanged))
            .push(self.y0_input.view().map(MyFormMessage::Y0Changed))
            .push(button)
            .push(reset_btn)
            .into()
    }

    pub fn subscription(&self) -> Subscription<MyFormMessage> {
        Subscription::none()
    }
}
