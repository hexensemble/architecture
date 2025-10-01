use crate::core::application::*;
use crate::core::state_manager::*;

mod app;
mod core;

fn main() {
    let app_spec = ApplicationSpecification {
        title: "Game Idea".to_string(),
        width: 800,
        height: 600,
        fps: 60,
    };

    let mut app = Application::new(app_spec);

    let mut state_manager = StateManager::new();

    app.run(&mut state_manager);
}
