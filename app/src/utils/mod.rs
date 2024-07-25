use directories::ProjectDirs;

pub mod json;

pub fn get_app_dirs() -> ProjectDirs {
    ProjectDirs::from("com", "", "fusion-launcher").unwrap()
}