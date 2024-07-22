use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use glob::glob;

fn main() -> Result<(), Box<dyn Error>> {
    let compiled_css_path = "./assets/styles/compiled.css";
    let mut compiled_css_file = File::create(compiled_css_path)?;

    let mut content = String::new();
    for entry in glob("./assets/styles/**/*.css").expect("Failed to find CSS files") {
        let path = entry?;
        // Ignore the compiled CSS file
        if path.to_str().unwrap() == compiled_css_path {
            continue;
        }
        let data = fs::read_to_string(path)?;
        content += data.as_str();
    }

    compiled_css_file.write_all(content.as_bytes())?;
    compiled_css_file.flush()?;

    Ok(())
}