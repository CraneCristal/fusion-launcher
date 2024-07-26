use crate::error::Error;
use crate::model::game::Game;
use crate::utils::dir::get_app_dirs;
use crate::utils::json::{deserialize_from_file, serialize_to_file};
use std::fs;
use std::fs::File;
use std::path::PathBuf;

const GAMES_FILE_NAME: &str = "games.json";

/// Return the games.json file path in app data directory.
///
/// Create the file if it doesn't exist.
fn get_games_file_path() -> PathBuf {
    let app_data_dir = get_app_dirs().data_dir().to_path_buf();
    let games_file_path = app_data_dir.join(GAMES_FILE_NAME);
    if !games_file_path.exists() {
        fs::create_dir_all(app_data_dir).expect("Failed to create data directory");
        File::create(&games_file_path).expect("Failed to create games.json file");
    }

    games_file_path
}

/// Read and deserialize the games in the user library
///
/// If the deserialization is impossible, save and return an empty vector.
pub fn get_games() -> Result<Vec<Game>, Error> {
    let games_file_path = get_games_file_path();
    match deserialize_from_file(games_file_path) {
        Ok(games) => Ok(games),
        Err(error) => {
            match error {
                // Json file corrupted
                Error::Json(_) => {
                    save_games(vec![])?;
                    Ok(Vec::new())
                }
                _ => Err(error),
            }
        }
    }
}

/// Serialize and write games in the user library
pub fn save_games(games: Vec<Game>) -> Result<(), Error> {
    let games_file_path = get_games_file_path();
    serialize_to_file(games, games_file_path)
}
