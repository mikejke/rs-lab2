#![windows_subsystem = "windows"]
pub mod components;

use components::{MyChart, MyChartMessage, MyForm, MyFormMessage};
use iced::widget::{Column, Container, Row};
use iced::{executor, Result, Settings};
use iced::{Alignment, Application, Command, Element, Length, Theme};

fn main() -> Result {
    App::run(Settings {
        antialiasing: true,
        ..Default::default()
    })
}

#[derive(Debug)]
enum Message {
    FormUpdate(MyFormMessage),
    ChartEvent(MyChartMessage),
}

struct App {
    chart: MyChart,
    form: MyForm,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let initial_data: (f32, f32, f32, usize) = (0.0, 5.0, 1.0, 100);

        (
            Self {
                chart: MyChart::new(
                    "y' = x*sin(xy)",
                    initial_data,
                    Box::new(|x, y| x * (x * y).sin()),
                ),
                form: MyForm::new(initial_data),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Лабораторная работа 2".to_owned()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::FormUpdate(msg) => match msg {
                MyFormMessage::SubmitEvent(data) => {
                    let chart_data: (f32, f32, f32, usize) = (
                        data.begin,
                        data.end,
                        data.y0,
                        data.count,
                    );
                    self.form.update(MyFormMessage::SubmitEvent(data));
                    self.chart.update(MyChartMessage::ChartUpdate(chart_data));
                }
                MyFormMessage::ResetEvent => {
                    self.form.update(MyFormMessage::ResetEvent);
                    self.chart.update(MyChartMessage::ChartClear);
                }
                msg => self.form.update(msg),
            },
            _ => {
                // do nothing
            },
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let left = Column::new()
            .align_items(Alignment::Start)
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .push(self.form.view().map(Message::FormUpdate));

        let right = Column::new()
            .align_items(Alignment::Start)
            .width(Length::FillPortion(7))
            .height(Length::Fill)
            .push(self.chart.view().map(Message::ChartEvent));

        let content = Row::new()
            .spacing(10)
            .align_items(Alignment::Start)
            .width(Length::Fill)
            .height(Length::Fill)
            .push(left)
            .push(right);

        Container::new(content)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
