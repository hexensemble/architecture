use crate::engine::action::*;
use crate::engine::application::*;
use crate::engine::logging::*;
use crate::engine::settings::*;
use std::fs;
use std::path::PathBuf;
use std::process;

mod app;
mod engine;
mod game;
mod net;

const SETTINGS: &str = "settings.json";

fn main() {
    match init_logging("application.log".to_string()) {
        Ok(_) => {}
        Err(e) => eprintln!("Failed to initialize logging: {e}"),
    }

    log::info!("Starting application...");

    match load_settings() {
        Ok(settings) => {
            let mut app = Application::new(settings);

            match app.run(crate::app::initial_layer()) {
                Ok(()) => {
                    log::info!("Application ran successfully!");
                    process::exit(0)
                }
                Err(e) => {
                    log::error!("Error! {e}");
                    process::exit(1)
                }
            }
        }
        Err(e) => {
            log::error!("Error! {e}");
            process::exit(1);
        }
    }
}

fn load_settings<A: ActionType>() -> Result<Settings<A>, Box<dyn std::error::Error>> {
    log::info!("Loading settings...");

    let path = PathBuf::from(SETTINGS);

    if !path.exists() {
        log::warn!("Unable to locate '{SETTINGS}', creating defaults...");

        let default_settings = Settings::default();
        let data = serde_json::to_string_pretty(&default_settings)?;

        log::info!("Done!");
        log::info!("Saving {SETTINGS}...");

        fs::write(&path, data)?;

        log::info!("Success!");

        return Ok(default_settings);
    }

    let data = fs::read_to_string(&path)?;
    let settings: Settings<A> = serde_json::from_str(&data)?;

    log::info!("Success!");

    Ok(settings)
}
