const APP_STYLE: &str = include_str!("../../assets/styles/compiled.css");

pub fn get_app_style() -> String {
    APP_STYLE.to_string()
}