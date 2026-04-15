use crate::engine::action::*;
use crate::engine::application::*;
use crate::engine::logging::*;
use crate::engine::settings::*;
use std::fs;
use std::path::PathBuf;

mod app;
mod engine;
mod game;
mod net;

const SETTINGS_FILE: &str = "settings.json";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging("application.log".to_string())?;

    log::info!("Starting application");

    let settings = load_settings()?;

    let mut app = Application::new(settings);

    app.run(crate::app::initial_layer())?;

    log::info!("Application ran successfully");

    Ok(())
}

fn load_settings<A: ActionType>() -> Result<Settings<A>, Box<dyn std::error::Error>> {
    log::info!("Loading settings");

    let path = PathBuf::from(SETTINGS_FILE);

    if !path.exists() {
        log::warn!("Unable to locate settings file: {SETTINGS_FILE}, creating defaults");

        let default_settings = Settings::default();
        let data = serde_json::to_string_pretty(&default_settings)?;

        log::info!("Saving default settings as: {SETTINGS_FILE}");

        fs::write(&path, data)?;

        log::info!("Settings file saved");

        return Ok(default_settings);
    }

    let data = fs::read_to_string(&path)?;
    let settings: Settings<A> = serde_json::from_str(&data)?;

    log::info!("Successfully loaded settings");

    Ok(settings)
}
