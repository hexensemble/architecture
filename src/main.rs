use crate::core::application::*;
use std::fs;
use std::path::PathBuf;

mod app;
mod core;

const APP_SPEC_FILENAME: &str = "app-spec.json";

fn main() {
    match load_app_spec() {
        Ok(app_spec) => {
            let mut app = Application::new(app_spec);

            app.run(crate::app::initial_layer(), crate::app::bindings());
        }
        Err(e) => {
            eprintln!("Error! {e}");
        }
    }
}

fn load_app_spec() -> Result<ApplicationSpecification, Box<dyn std::error::Error>> {
    println!("Loading Application Specification...");

    let path = PathBuf::from(APP_SPEC_FILENAME);

    if !path.exists() {
        println!("Unable to locate '{APP_SPEC_FILENAME}', creating defaults...");

        let default_spec = ApplicationSpecification {
            title: "Untitled".to_string(),
            width: 800,
            height: 600,
            fps: 30,
        };
        let data = serde_json::to_string_pretty(&default_spec)?;

        println!("Done!");
        println!("Saving {APP_SPEC_FILENAME}...");

        fs::write(&path, data)?;

        println!("Success!");

        return Ok(default_spec);
    }

    let data = fs::read_to_string(&path)?;
    let app_spec: ApplicationSpecification = serde_json::from_str(&data)?;

    println!("Success!");

    Ok(app_spec)
}
