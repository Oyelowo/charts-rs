use crate::charts::measure_text_width_family;

use super::color::*;
use super::common::*;
use super::component::*;
use super::util::*;
use super::Canvas;

#[derive(Clone, Debug, Default)]
pub struct LineChart {
    pub width: f64,
    pub height: f64,
    pub margin: Box,
    pub series_list: Vec<Series>,
    pub font_family: String,
    pub background_color: Color,

    // title
    pub title_text: String,
    pub title_font_size: f64,
    pub title_font_color: Color,
    pub title_font_weight: Option<String>,
    pub title_margin: Option<Box>,

    // x axis
    pub x_axis_data: Vec<String>,
    pub x_axis_height: f64,
    pub x_axis_stroke_color: Color,
    pub x_axis_font_size: f64,
    pub x_axis_font_color: Color,
    pub x_axis_name_gap: f64,
    pub x_axis_name_rotate: f64,
    // y axis
    pub y_axis_font_size: f64,
    pub y_axis_font_color: Color,
    pub y_axis_width: f64,
    pub y_axis_split_number: usize,
    pub y_axis_name_gap: f64,

    // grid
    pub grid_stroke_color: Color,
    pub grid_stroke_width: f64,

    // series
    pub series_stroke_width: f64,
    pub series_colors: Vec<Color>,
    pub series_symbol: Option<Symbol>,
    pub series_smooth: bool,
    pub series_fill: bool,
}

impl LineChart {
    pub fn new(series_list: Vec<Series>, x_axis_data: Vec<String>) -> LineChart {
        let mut l = LineChart {
            series_list,
            x_axis_data,
            ..Default::default()
        };
        l.fill_theme("".to_string());
        l
    }
    pub fn fill_theme(&mut self, theme: String) {
        let t = get_theme(theme);

        self.font_family = t.font_family;
        self.margin = t.margin;
        self.width = t.width;
        self.height = t.height;
        self.background_color = t.background_color;

        self.title_font_color = t.title_font_color;
        self.title_font_size = t.title_font_size;
        self.title_font_weight = t.title_font_weight;
        self.title_margin = t.title_margin;

        self.x_axis_font_size = t.x_axis_font_size;
        self.x_axis_font_color = t.x_axis_font_color;
        self.x_axis_stroke_color = t.x_axis_stroke_color;
        self.x_axis_name_gap = t.x_axis_name_gap;
        self.x_axis_height = t.x_axis_height;

        self.y_axis_font_color = t.y_axis_font_color;
        self.y_axis_font_size = t.y_axis_font_size;
        self.y_axis_width = t.y_axis_width;
        self.y_axis_split_number = t.y_axis_split_number;
        self.y_axis_name_gap = t.y_axis_name_gap;

        self.grid_stroke_color = t.grid_stroke_color;
        self.grid_stroke_width = t.grid_stroke_width;

        self.series_colors = t.series_colors;
        self.series_stroke_width = t.series_stroke_width;

        self.series_symbol = Some(Symbol::Circle(
            self.series_stroke_width,
            Some(self.background_color),
        ));
    }
    pub fn svg(&self) {
        let mut c = Canvas::new(self.width, self.height);
        c.margin = self.margin.clone();

        let mut axis_top = 0.0;

        if !self.title_text.is_empty() {
            let title_margin = self.title_margin.clone().unwrap_or_default();
            let b = c.child(title_margin).text(Text {
                text: self.title_text.clone(),
                font_family: Some(self.font_family.clone()),
                font_size: Some(self.title_font_size),
                font_weight: self.title_font_weight.clone(),
                fill: Some(self.title_font_color),
                y: Some(self.title_font_size),
                ..Default::default()
            });
            axis_top = b.outer_height();
        }

        let axis_height = c.height() - self.x_axis_height - axis_top;
        let axis_width = c.width() - self.y_axis_width;
        // 顶部文本区域
        if axis_top > 0.0 {
            c = c.child(Box {
                top: axis_top,
                ..Default::default()
            });
        }

        c.grid(Grid {
            left: self.y_axis_width,
            right: self.y_axis_width + axis_width,
            bottom: axis_height,
            color: Some(self.grid_stroke_color),
            stroke_width: self.grid_stroke_width,
            horizontals: self.y_axis_split_number,
            hidden_horizontals: vec![self.y_axis_split_number],
            ..Default::default()
        });

        let mut data_list = vec![];
        for series in self.series_list.iter() {
            data_list.append(series.data.clone().as_mut());
        }
        let y_axis_values = get_axis_values(AxisValueParams {
            data_list,
            split_number: self.y_axis_split_number,
            reverse: Some(true),
            ..Default::default()
        });
        // y axis
        c.axis(Axis {
            position: Position::Left,
            height: axis_height,
            width: self.y_axis_width,
            split_number: self.y_axis_split_number,
            font_family: self.font_family.clone(),
            stroke_color: Some((0, 0, 0, 0).into()),
            name_align: Align::Left,
            name_gap: self.y_axis_name_gap,
            font_color: Some(self.y_axis_font_color),
            font_size: self.y_axis_font_size,
            data: y_axis_values.data.clone(),
            ..Default::default()
        });

        // x axis
        c.child(Box {
            top: c.height() - self.x_axis_height,
            left: self.y_axis_width,
            ..Default::default()
        })
        .axis(Axis {
            height: self.x_axis_height,
            width: axis_width,
            split_number: self.x_axis_data.len(),
            font_family: self.font_family.clone(),
            data: self.x_axis_data.clone(),
            font_color: Some(self.x_axis_font_color),
            stroke_color: Some(self.x_axis_stroke_color),
            font_size: self.x_axis_font_size,
            name_gap: self.x_axis_name_gap,
            name_rotate: self.x_axis_name_rotate,
            ..Default::default()
        });

        // line point
        let max_height = c.height() - self.x_axis_height;

        let mut series_canvas = c.child(Box {
            left: self.y_axis_width,
            ..Default::default()
        });
        for (index, series) in self.series_list.iter().enumerate() {
            let unit_width = series_canvas.width() / series.data.len() as f64;
            let mut points: Vec<Point> = vec![];
            for (i, p) in series.data.iter().enumerate() {
                // 居中
                let x = unit_width * i as f64 + unit_width / 2.0;
                let y = y_axis_values.get_offset_height(p.to_owned(), max_height);
                points.push((x, y).into());
            }

            let color = *self
                .series_colors
                .get(index)
                .unwrap_or_else(|| &self.series_colors[0]);

            let fill = color.with_alpha(100);
            let series_fill = self.series_fill;
            if self.series_smooth {
                if series_fill {
                    series_canvas.smooth_line_fill(SmoothLineFill {
                        fill,
                        points: points.clone(),
                        bottom: axis_height,
                    });
                }
                series_canvas.smooth_line(SmoothLine {
                    points,
                    color: Some(color),
                    stroke_width: self.series_stroke_width,
                    symbol: self.series_symbol.clone(),
                });
            } else {
                if series_fill {
                    series_canvas.straight_line_fill(StraightLineFill {
                        fill,
                        points: points.clone(),
                        bottom: axis_height,
                    });
                }
                series_canvas.straight_line(StraightLine {
                    points,
                    color: Some(color),
                    stroke_width: self.series_stroke_width,
                    symbol: self.series_symbol.clone(),
                });
            }
        }

        println!("{}", c.svg().unwrap())
    }
}