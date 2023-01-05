use iced::{Element, Length};
use plotters::prelude::*;
use plotters_backend::DrawingBackend;
use plotters_iced::{Chart, ChartWidget};

#[allow(unused)]
pub struct MyChart {
    series_collection: Vec<(MyChartData, RGBColor)>,
    func: Box<dyn Fn(f32, f32) -> f32>,
    caption: String,
}

#[derive(Debug, Clone)]
pub struct MyChartData {
    pub begin: f32,
    pub end: f32,
    pub y0: f32,
    pub count: usize,
}

#[derive(Debug, Clone)]
pub enum MyChartMessage {
    ChartUpdate((f32, f32, f32, usize)),
    ChartClear,
}

impl MyChart {
    pub fn new(
        caption: &str,
        initial: (f32, f32, f32, usize),
        func: Box<dyn Fn(f32, f32) -> f32>,
    ) -> Self {
        let (begin, end, y0, count) = initial;
        let color = RGBColor(
            rand::random::<u8>(),
            rand::random::<u8>(),
            rand::random::<u8>(),
        );
        Self {
            caption: caption.to_string(),
            series_collection: vec![(
                MyChartData {
                    begin,
                    end,
                    y0,
                    count,
                },
                color,
            )],
            func,
        }
    }

    pub fn update(&mut self, message: MyChartMessage) {
        match message {
            MyChartMessage::ChartUpdate(data) => {
                let (begin, end, y0, count) = data;
                println!("Updating chart with data: {:?}", data);
                let color = RGBColor(
                    rand::random::<u8>(),
                    rand::random::<u8>(),
                    rand::random::<u8>(),
                );
                self.series_collection.push((
                    MyChartData {
                        begin,
                        end,
                        y0,
                        count,
                    },
                    color,
                ));
            }
            MyChartMessage::ChartClear => {
                println!("Clearing chart");
                self.series_collection = vec![];
            }
        }
    }

    pub fn view(&self) -> Element<MyChartMessage> {
        let chart = ChartWidget::new(self)
            .width(Length::Fill)
            .height(Length::Fill);

        chart.into()
    }
}

impl Chart<MyChartMessage> for MyChart {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut builder: ChartBuilder<DB>) {
        // recalculate chart x specs
        let x_spec = (
            self.series_collection
                .iter()
                .map(|(series, _)| series.begin)
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(0.0),
            self.series_collection
                .iter()
                .map(|(series, _)| series.end)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(0.0),
        );

        // recalculate chart y specs
        let y_spec = (
            self.series_collection
                .iter()
                .map(|(series, _)| (series.y0))
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(0.0),
            self.series_collection
                .iter()
                .map(|(series, _)| series.y0 + 1.0)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(0.0)
        );
        

        let mut chart = builder
            .caption(self.caption.as_str(), ("sans-serif", 32.0).into_font())
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x_spec.0..x_spec.1, y_spec.0..y_spec.1)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        for (i, (series, color)) in self.series_collection.iter().enumerate() {
            let mut y = series.y0;
            let step = (series.end - series.begin) / series.count as f32;

            let mut x = series.begin;
            let mut s: Vec<(f32, f32)> = vec![];
            while x < series.end {
                let k1 = (self.func)(x, y);
                let k2 = (self.func)(x + step / 2.0, y + step * k1 / 2.0);
                let k3 = (self.func)(x + step / 2.0, y + step * k2 / 2.0);
                let k4 = (self.func)(x + step, y + step * k3);
                y += step * (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;

                s.push((x, y));
                x += step;
            }

            let color = color.clone();

            chart
                .draw_series(LineSeries::new(s, color.stroke_width(3)))
                .unwrap()
                .label(format!("Series {}", i + 1))
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &color));
        }
        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()
            .unwrap();
    }
}
