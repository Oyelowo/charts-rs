use super::color::Color;
use super::common::Align;
use super::font::DEFAULT_FONT_FAMILY;
use super::util::Box;
use once_cell::sync::{Lazy, OnceCell};

pub static DEFAULT_WIDTH: f32 = 600.0;
pub static DEFAULT_HEIGHT: f32 = 400.0;

pub static DEFAULT_TITLE_HEIGHT: f32 = 30.0;
pub static DEFAULT_SUB_TITLE_HEIGHT: f32 = 20.0;

pub static DEFAULT_X_AXIS_HEIGHT: f32 = 30.0;
pub static DEFAULT_X_AXIS_NAME_GAP: f32 = 5.0;

pub static DEFAULT_Y_AXIS_WIDTH: f32 = 40.0;
pub static DEFAULT_Y_AXIS_NAME_GAP: f32 = 8.0;
pub static DEFAULT_Y_AXIS_SPLIT_NUMBER: usize = 6;
pub static DEFAULT_FONT_SIZE: f32 = 14.0;

pub static DEFAULT_SERIES_STROKE_WIDTH: f32 = 2.0;

pub static THEME_DARK: &str = "dark";
pub static THEME_ANT: &str = "ant";
pub static THEME_GRAFANA: &str = "grafana";

static E_CHART: &str = "echart";

pub fn get_or_init_default_theme(theme: &str) -> String {
    static DEFAULT_THEME: OnceCell<String> = OnceCell::new();
    let value = DEFAULT_THEME.get_or_init(|| {
        if theme.is_empty() {
            return E_CHART.to_string();
        }
        theme.to_string()
    });
    value.to_owned()
}

pub fn get_default_theme() -> String {
    get_or_init_default_theme(E_CHART)
}

#[derive(Clone, Debug, Default)]

pub struct Theme {
    pub is_light: bool,
    pub font_family: String,
    pub margin: Box,
    pub width: f32,
    pub height: f32,
    pub background_color: Color,

    // title
    pub title_font_size: f32,
    pub title_font_color: Color,
    pub title_font_weight: Option<String>,
    pub title_margin: Option<Box>,
    pub title_align: Align,
    pub title_height: f32,

    // sub title
    pub sub_title_font_size: f32,
    pub sub_title_font_color: Color,
    pub sub_title_margin: Option<Box>,
    pub sub_title_align: Align,
    pub sub_title_height: f32,

    // legend
    pub legend_font_size: f32,
    pub legend_font_color: Color,
    pub legend_align: Align,
    pub legend_margin: Option<Box>,

    // x axis
    pub x_axis_font_size: f32,
    pub x_axis_stroke_color: Color,
    pub x_axis_font_color: Color,
    pub x_axis_name_gap: f32,
    pub x_axis_height: f32,

    // y axis
    pub y_axis_font_size: f32,
    pub y_axis_font_color: Color,
    pub y_axis_stroke_color: Color,
    pub y_axis_split_number: usize,
    pub y_axis_name_gap: f32,

    // grid
    pub grid_stroke_color: Color,
    pub grid_stroke_width: f32,

    // series
    pub series_stroke_width: f32,
    pub series_label_font_size: f32,
    pub series_label_font_color: Color,
    pub series_colors: Vec<Color>,

    // table
    pub table_header_color: Color,
    pub table_body_colors: Vec<Color>,
    pub table_border_color: Color,
}

static LIGHT_THEME: Lazy<Theme> = Lazy::new(|| {
    let x_axis_color = (110, 112, 121).into();
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
        title_margin: None,
        title_align: Align::Center,
        title_height: DEFAULT_TITLE_HEIGHT,

        sub_title_font_color: font_color,
        sub_title_font_size: DEFAULT_FONT_SIZE,
        sub_title_margin: None,
        sub_title_align: Align::Center,
        sub_title_height: DEFAULT_SUB_TITLE_HEIGHT,

        legend_font_size: DEFAULT_FONT_SIZE,
        legend_font_color: font_color,
        legend_align: Align::Center,
        legend_margin: None,

        x_axis_font_size: DEFAULT_FONT_SIZE,
        x_axis_stroke_color: x_axis_color,
        x_axis_font_color: x_axis_color,
        x_axis_name_gap: DEFAULT_X_AXIS_NAME_GAP,
        x_axis_height: DEFAULT_X_AXIS_HEIGHT,

        y_axis_font_size: DEFAULT_FONT_SIZE,
        y_axis_font_color: x_axis_color,
        y_axis_stroke_color: Color::transparent(),
        y_axis_split_number: DEFAULT_Y_AXIS_SPLIT_NUMBER,
        y_axis_name_gap: DEFAULT_Y_AXIS_NAME_GAP,

        grid_stroke_color: (224, 230, 242).into(),
        grid_stroke_width: 1.0,

        series_stroke_width: DEFAULT_SERIES_STROKE_WIDTH,
        series_label_font_size: DEFAULT_FONT_SIZE,
        series_label_font_color: font_color,
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

        table_header_color: (242, 243, 245).into(),
        table_body_colors: vec![(255, 255, 255).into()],
        table_border_color: (229, 230, 235).into(),
    }
});

static DARK_THEME: Lazy<Theme> = Lazy::new(|| {
    let x_axis_color = (185, 184, 206).into();
    let bg_color = (16, 12, 42).into();

    let font_color: Color = (238, 238, 238).into();
    Theme {
        is_light: false,
        font_family: DEFAULT_FONT_FAMILY.to_string(),
        margin: (5.0).into(),
        width: DEFAULT_WIDTH,
        height: DEFAULT_HEIGHT,
        background_color: bg_color,

        title_font_color: font_color,
        title_font_size: 18.0,
        title_font_weight: Some("bold".to_string()),
        title_margin: None,
        title_align: Align::Center,
        title_height: DEFAULT_TITLE_HEIGHT,

        sub_title_font_color: font_color,
        sub_title_font_size: DEFAULT_FONT_SIZE,
        sub_title_margin: None,
        sub_title_align: Align::Center,
        sub_title_height: DEFAULT_SUB_TITLE_HEIGHT,

        legend_font_size: DEFAULT_FONT_SIZE,
        legend_font_color: font_color,
        legend_align: Align::Center,
        legend_margin: None,

        x_axis_font_size: DEFAULT_FONT_SIZE,
        x_axis_stroke_color: x_axis_color,
        x_axis_font_color: x_axis_color,
        x_axis_name_gap: DEFAULT_X_AXIS_NAME_GAP,
        x_axis_height: DEFAULT_X_AXIS_HEIGHT,

        y_axis_font_size: DEFAULT_FONT_SIZE,
        y_axis_font_color: x_axis_color,
        y_axis_stroke_color: Color::transparent(),
        y_axis_split_number: DEFAULT_Y_AXIS_SPLIT_NUMBER,
        y_axis_name_gap: DEFAULT_Y_AXIS_NAME_GAP,

        grid_stroke_color: (71, 71, 83).into(),
        grid_stroke_width: 1.0,

        series_stroke_width: DEFAULT_SERIES_STROKE_WIDTH,
        series_label_font_size: DEFAULT_FONT_SIZE,
        series_label_font_color: font_color,
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

        table_header_color: bg_color,
        table_body_colors: vec![bg_color.with_alpha(230)],
        table_border_color: (100, 100, 100).into(),
    }
});

static ANT_THEME: Lazy<Theme> = Lazy::new(|| {
    let x_axis_color = (110, 112, 121).into();

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
        title_margin: None,
        title_align: Align::Center,
        title_height: DEFAULT_TITLE_HEIGHT,

        sub_title_font_color: font_color,
        sub_title_font_size: DEFAULT_FONT_SIZE,
        sub_title_margin: None,
        sub_title_align: Align::Center,
        sub_title_height: DEFAULT_SUB_TITLE_HEIGHT,

        legend_font_size: DEFAULT_FONT_SIZE,
        legend_font_color: font_color,
        legend_align: Align::Center,
        legend_margin: None,

        x_axis_font_size: DEFAULT_FONT_SIZE,
        x_axis_stroke_color: x_axis_color,
        x_axis_font_color: x_axis_color,
        x_axis_name_gap: DEFAULT_X_AXIS_NAME_GAP,
        x_axis_height: DEFAULT_X_AXIS_HEIGHT,

        y_axis_font_size: DEFAULT_FONT_SIZE,
        y_axis_font_color: x_axis_color,
        y_axis_stroke_color: Color::transparent(),
        y_axis_split_number: DEFAULT_Y_AXIS_SPLIT_NUMBER,
        y_axis_name_gap: DEFAULT_Y_AXIS_NAME_GAP,

        grid_stroke_color: (224, 230, 242).into(),
        grid_stroke_width: 1.0,

        series_stroke_width: DEFAULT_SERIES_STROKE_WIDTH,
        series_label_font_size: DEFAULT_FONT_SIZE,
        series_label_font_color: font_color,

        series_colors: vec![
            "#5b8ff9".into(),
            "#5ad8a6".into(),
            "#5d7092".into(),
            "#f6bd16".into(),
            "#6f5ef9".into(),
            "#6dc8ec".into(),
            "#945fb9".into(),
            "#ff9845".into(),
        ],

        table_header_color: (250, 250, 250).into(),
        table_body_colors: vec![(255, 255, 255).into()],
        table_border_color: (239, 239, 244).into(),
    }
});

static VINTAGE_THEME: Lazy<Theme> = Lazy::new(|| {
    let x_axis_color = (0, 0, 0).into();

    let font_color: Color = (51, 51, 51).into();
    Theme {
        is_light: true,
        font_family: DEFAULT_FONT_FAMILY.to_string(),
        margin: (5.0).into(),
        width: DEFAULT_WIDTH,
        height: DEFAULT_HEIGHT,
        background_color: (254, 248, 239).into(),

        title_font_color: font_color,
        title_font_size: 18.0,
        title_font_weight: Some("bold".to_string()),
        title_margin: None,
        title_align: Align::Center,
        title_height: DEFAULT_TITLE_HEIGHT,

        sub_title_font_color: font_color,
        sub_title_font_size: DEFAULT_FONT_SIZE,
        sub_title_margin: None,
        sub_title_align: Align::Center,
        sub_title_height: DEFAULT_SUB_TITLE_HEIGHT,

        legend_font_size: DEFAULT_FONT_SIZE,
        legend_font_color: font_color,
        legend_align: Align::Center,
        legend_margin: None,

        x_axis_font_size: DEFAULT_FONT_SIZE,
        x_axis_stroke_color: x_axis_color,
        x_axis_font_color: x_axis_color,
        x_axis_name_gap: DEFAULT_X_AXIS_NAME_GAP,
        x_axis_height: DEFAULT_X_AXIS_HEIGHT,

        y_axis_font_size: DEFAULT_FONT_SIZE,
        y_axis_font_color: x_axis_color,
        y_axis_stroke_color: Color::transparent(),
        y_axis_split_number: DEFAULT_Y_AXIS_SPLIT_NUMBER,
        y_axis_name_gap: DEFAULT_Y_AXIS_NAME_GAP,

        grid_stroke_color: (224, 230, 242).into(),
        grid_stroke_width: 1.0,

        series_stroke_width: DEFAULT_SERIES_STROKE_WIDTH,
        series_label_font_size: DEFAULT_FONT_SIZE,
        series_label_font_color: font_color,

        series_colors: vec![
            "#d87c7c".into(),
            "#919e8b".into(),
            "#d7ab82".into(),
            "#6e7074".into(),
            "#61a0a8".into(),
            "#efa18d".into(),
            "#787464".into(),
            "#cc7e63".into(),
            "#724e58".into(),
            "#4b565b".into(),
        ],

        table_header_color: (250, 250, 250).into(),
        table_body_colors: vec![(255, 255, 255).into()],
        table_border_color: (239, 239, 244).into(),
    }
});

static SHINE_THEME: Lazy<Theme> = Lazy::new(|| {
    let x_axis_color = (0, 0, 0).into();

    let font_color: Color = (51, 51, 51).into();
    Theme {
        is_light: true,
        font_family: DEFAULT_FONT_FAMILY.to_string(),
        margin: (5.0).into(),
        width: DEFAULT_WIDTH,
        height: DEFAULT_HEIGHT,
        background_color: (254, 248, 239).into(),

        title_font_color: font_color,
        title_font_size: 18.0,
        title_font_weight: Some("bold".to_string()),
        title_margin: None,
        title_align: Align::Center,
        title_height: DEFAULT_TITLE_HEIGHT,

        sub_title_font_color: font_color,
        sub_title_font_size: DEFAULT_FONT_SIZE,
        sub_title_margin: None,
        sub_title_align: Align::Center,
        sub_title_height: DEFAULT_SUB_TITLE_HEIGHT,

        legend_font_size: DEFAULT_FONT_SIZE,
        legend_font_color: font_color,
        legend_align: Align::Center,
        legend_margin: None,

        x_axis_font_size: DEFAULT_FONT_SIZE,
        x_axis_stroke_color: x_axis_color,
        x_axis_font_color: x_axis_color,
        x_axis_name_gap: DEFAULT_X_AXIS_NAME_GAP,
        x_axis_height: DEFAULT_X_AXIS_HEIGHT,

        y_axis_font_size: DEFAULT_FONT_SIZE,
        y_axis_font_color: x_axis_color,
        y_axis_stroke_color: Color::transparent(),
        y_axis_split_number: DEFAULT_Y_AXIS_SPLIT_NUMBER,
        y_axis_name_gap: DEFAULT_Y_AXIS_NAME_GAP,

        grid_stroke_color: (224, 230, 242).into(),
        grid_stroke_width: 1.0,

        series_stroke_width: DEFAULT_SERIES_STROKE_WIDTH,
        series_label_font_size: DEFAULT_FONT_SIZE,
        series_label_font_color: font_color,

        series_colors: vec![
            "#c12e34".into(),
            "#e6b600".into(),
            "#0098d9".into(),
            "#2b821d".into(),
            "#005eaa".into(),
            "#339ca8".into(),
            "#cda819".into(),
            "#32a487".into(),
        ],

        table_header_color: (250, 250, 250).into(),
        table_body_colors: vec![(255, 255, 255).into()],
        table_border_color: (239, 239, 244).into(),
    }
});

static GRAFANA_THEME: Lazy<Theme> = Lazy::new(|| {
    let x_axis_color = (185, 184, 206).into();

    let font_color: Color = (216, 217, 218).into();
    let bg_color = (31, 29, 29).into();
    Theme {
        is_light: false,
        font_family: DEFAULT_FONT_FAMILY.to_string(),
        margin: (5.0).into(),
        width: DEFAULT_WIDTH,
        height: DEFAULT_HEIGHT,
        background_color: bg_color,

        title_font_color: font_color,
        title_font_size: 18.0,
        title_font_weight: Some("bold".to_string()),
        title_margin: None,
        title_align: Align::Center,
        title_height: DEFAULT_TITLE_HEIGHT,

        sub_title_font_color: font_color,
        sub_title_font_size: DEFAULT_FONT_SIZE,
        sub_title_margin: None,
        sub_title_align: Align::Center,
        sub_title_height: DEFAULT_SUB_TITLE_HEIGHT,

        legend_font_size: DEFAULT_FONT_SIZE,
        legend_font_color: font_color,
        legend_align: Align::Center,
        legend_margin: None,

        x_axis_font_size: DEFAULT_FONT_SIZE,
        x_axis_stroke_color: x_axis_color,
        x_axis_font_color: x_axis_color,
        x_axis_name_gap: DEFAULT_X_AXIS_NAME_GAP,
        x_axis_height: DEFAULT_X_AXIS_HEIGHT,

        y_axis_font_size: DEFAULT_FONT_SIZE,
        y_axis_font_color: x_axis_color,
        y_axis_stroke_color: Color::transparent(),
        y_axis_split_number: DEFAULT_Y_AXIS_SPLIT_NUMBER,
        y_axis_name_gap: DEFAULT_Y_AXIS_NAME_GAP,

        grid_stroke_color: (68, 67, 67).into(),
        grid_stroke_width: 1.0,

        series_stroke_width: DEFAULT_SERIES_STROKE_WIDTH,
        series_label_font_size: DEFAULT_FONT_SIZE,
        series_label_font_color: font_color,

        series_colors: vec![
            "#7EB26D".into(),
            "#EAB839".into(),
            "#6ED0E0".into(),
            "#EF843C".into(),
            "#E24D42".into(),
            "#1F78C1".into(),
            "#705DA0".into(),
            "#508642".into(),
        ],

        table_header_color: bg_color,
        table_body_colors: vec![bg_color.with_alpha(230)],
        table_border_color: (239, 239, 244).into(),
    }
});

pub fn get_theme(theme: &str) -> &'static Theme {
    match theme {
        "dark" => &DARK_THEME,
        "ant" => &ANT_THEME,
        "grafana" => &GRAFANA_THEME,
        "vintage" => &VINTAGE_THEME,
        "shine" => &SHINE_THEME,
        // echart
        _ => &LIGHT_THEME,
    }
}
