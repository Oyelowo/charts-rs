use super::color::Color;
use super::font::DEFAULT_FONT_FAMILY;
use super::util::Box;
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::sync::Mutex;

pub static DEFAULT_WIDTH: f64 = 600.0;
pub static DEFAULT_HEIGHT: f64 = 400.0;

pub static DEFAULT_X_AXIS_HEIGHT: f64 = 30.0;
pub static DEFAULT_X_AXIS_NAME_GAP: f64 = 5.0;

pub static DEFAULT_Y_AXIS_WIDTH: f64 = 40.0;
pub static DEFAULT_Y_AXIS_NAME_GAP: f64 = 8.0;
pub static DEFAULT_Y_AXIS_SPLIT_NUMBER: usize = 6;
pub static DEFAULT_FONT_SIZE: f64 = 14.0;

pub static DEFAULT_SERIES_STROKE_WIDTH: f64 = 2.0;

static E_CHART: &str = "echart";
static DEFAULT_THEME: Lazy<Mutex<RefCell<String>>> =
    Lazy::new(|| Mutex::new(RefCell::new(E_CHART.to_string())));

pub fn get_default_theme() -> String {
    if let Ok(value) = DEFAULT_THEME.lock() {
        value.borrow().to_string()
    } else {
        E_CHART.to_string()
    }
}

pub fn set_default_theme(theme: String) -> String {
    if let Ok(value) = DEFAULT_THEME.lock() {
        *value.borrow_mut() = theme.clone();
        theme
    } else {
        "".to_string()
    }
}

#[derive(Clone, Debug, Default)]

pub struct Theme {
    pub is_light: bool,
    pub font_family: String,
    pub margin: Box,
    pub width: f64,
    pub height: f64,
    pub background_color: Color,

    // title
    pub title_font_size: f64,
    pub title_font_color: Color,
    pub title_font_weight: Option<String>,
    pub title_margin: Option<Box>,

    // legend
    pub legend_font_size: f64,
    pub legend_font_color: Color,

    // x axis
    pub x_axis_font_size: f64,
    pub x_axis_stroke_color: Color,
    pub x_axis_font_color: Color,
    pub x_axis_name_gap: f64,
    pub x_axis_height: f64,

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
}

static LIGHT_THEME: Lazy<Theme> = Lazy::new(|| {
    let black = (110, 112, 121).into();
    let font_color: Color = (70, 70, 70).into();
    Theme {
        is_light: true,
        font_family: DEFAULT_FONT_FAMILY.to_string(),
        margin: (5.0).into(),
        width: DEFAULT_WIDTH,
        height: DEFAULT_HEIGHT,
        background_color: Color::white(),

        title_font_color: font_color,
        title_font_size: 18.0,
        title_font_weight: Some("bold".to_string()),
        title_margin: Some((10.0, 5.0, 10.0, 5.0).into()),

        legend_font_size: 14.0,
        legend_font_color: font_color,

        x_axis_font_size: DEFAULT_FONT_SIZE,
        x_axis_stroke_color: black,
        x_axis_font_color: black,
        x_axis_name_gap: DEFAULT_X_AXIS_NAME_GAP,
        x_axis_height: DEFAULT_X_AXIS_HEIGHT,

        y_axis_font_size: DEFAULT_FONT_SIZE,
        y_axis_font_color: black,
        y_axis_width: DEFAULT_Y_AXIS_WIDTH,
        y_axis_split_number: DEFAULT_Y_AXIS_SPLIT_NUMBER,
        y_axis_name_gap: DEFAULT_Y_AXIS_NAME_GAP,

        grid_stroke_color: (224, 230, 242).into(),
        grid_stroke_width: 1.0,

        series_stroke_width: DEFAULT_SERIES_STROKE_WIDTH,
        series_colors: vec![
            "#5470c6".into(),
            "#91cc75".into(),
            "#fac858".into(),
            "#ee6666".into(),
            "#73c0de".into(),
            "#3ba272".into(),
            "#fc8452".into(),
            "#9a60b4".into(),
            "#ea7ccc".into(),
        ],
    }
});

pub fn get_theme(theme: String) -> Theme {
    match &theme as &str {
        "black" => LIGHT_THEME.clone(),
        _ => LIGHT_THEME.clone(),
    }
}