use crate::app::layers::menu::MenuLayer;
use crate::core::application::*;

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

    app.set_initial_layer(Box::new(MenuLayer));
    app.run();
}
