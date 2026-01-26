use crate::core::action::*;
use crate::core::application::*;
use crate::core::settings::*;
use std::fs;
use std::path::PathBuf;
use std::process;

mod app;
mod core;

const SETTINGS: &str = "settings.json";

fn main() {
    match load_settings() {
        Ok(settings) => {
            let mut app = Application::new(settings);

            match app.run(crate::app::initial_layer()) {
                Ok(()) => process::exit(0),
                Err(e) => {
                    eprintln!("Error! {e}");
                    process::exit(1)
                }
            }
        }
        Err(e) => {
            eprintln!("Error! {e}");
            process::exit(1);
        }
    }
}

fn load_settings<A: ActionType>() -> Result<Settings<A>, Box<dyn std::error::Error>> {
    println!("Loading settings...");

    let path = PathBuf::from(SETTINGS);

    if !path.exists() {
        println!("Unable to locate '{SETTINGS}', creating defaults...");

        let default_settings = Settings::default();
        let data = serde_json::to_string_pretty(&default_settings)?;

        println!("Done!");
        println!("Saving {SETTINGS}...");

        fs::write(&path, data)?;

        println!("Success!");

        return Ok(default_settings);
    }

    let data = fs::read_to_string(&path)?;
    let settings: Settings<A> = serde_json::from_str(&data)?;

    println!("Success!");

    Ok(settings)
}
