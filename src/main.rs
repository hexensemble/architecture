use crate::core::application::*;
use std::fs;
use std::path::PathBuf;

mod app;
mod core;

fn main() {
    match load_app_spec() {
        Ok(app_spec) => {
            println!("Success!");

            let mut app = Application::new(app_spec);

            app.set_initial_layer(crate::app::initial_layer());
            app.run();
        }
        Err(e) => {
            eprintln!("Error! {e}");
        }
    }
}

fn load_app_spec() -> Result<ApplicationSpecification, Box<dyn std::error::Error>> {
    println!("Loading Application Specification...");

    let path = PathBuf::from("app-spec.json");

    if !path.exists() {
        println!("Unable to locate app-spec.json, creating defaults...");

        let default_spec = ApplicationSpecification {
            title: "Untitled".to_string(),
            width: 800,
            height: 600,
            fps: 30,
        };

        let data = serde_json::to_string_pretty(&default_spec)?;

        println!("Done!");
        println!("Saving app-spec.json...");

        fs::write(&path, data)?;

        println!("Saved!");

        return Ok(default_spec);
    }

    let data = fs::read_to_string(&path)?;
    let app_spec: ApplicationSpecification = serde_json::from_str(&data)?;

    Ok(app_spec)
}
