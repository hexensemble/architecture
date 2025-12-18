use crate::core::event::*;
use crate::core::layer::*;
use raylib::prelude::*;

pub struct ApplicationSpecification {
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub fps: u32,
}

pub struct Application {
    rl: RaylibHandle,
    thread: RaylibThread,
    layers: Vec<Box<dyn Layer>>,
    running: bool,
}

impl Application {
    pub fn new(spec: ApplicationSpecification) -> Self {
        let (mut rl, thread) = raylib::init()
            .size(spec.width, spec.height)
            .title(&spec.title)
            .build();

        rl.set_target_fps(spec.fps);

        Application {
            rl,
            thread,
            layers: Vec::new(),
            running: true,
        }
    }

    pub fn set_initial_layer(&mut self, mut layer: Box<dyn Layer>) {
        self.layers.clear();
        layer.on_attach();
        self.layers.push(layer);
    }

    pub fn run(&mut self) {
        if self.layers.is_empty() {
            eprintln!("Error: No initial layer set!");
            panic!();
        }

        while self.running {
            if self.rl.window_should_close() {
                self.stop();
                break;
            }

            // Events
            let events = collect_events(&self.rl);
            for event in events {
                for layer in self.layers.iter_mut().rev() {
                    if let Some(command) = layer.on_event(&event) {
                        self.handle_layer_command(command);
                        break;
                    }
                }
            }

            // Update
            if let Some(top_layer) = self.layers.last_mut() {
                top_layer.on_update(&mut self.rl);
            }

            if !self.running {
                continue;
            }

            // Render
            let mut d = self.rl.begin_drawing(&self.thread);
            d.clear_background(Color::WHITE);

            for layer in self.layers.iter_mut() {
                layer.on_render(&mut d);
            }
        }
    }

    fn handle_layer_command(&mut self, command: LayerCommand) {
        match command {
            LayerCommand::None => {}
            LayerCommand::Push(mut layer) => {
                layer.on_attach();
                self.layers.push(layer);
            }
            LayerCommand::Pop => {
                if let Some(mut layer) = self.layers.pop() {
                    layer.on_detach();
                }

                if self.layers.is_empty() {
                    self.stop();
                }
            }
            LayerCommand::Replace(mut layer) => {
                for mut old_layer in self.layers.drain(..) {
                    old_layer.on_detach();
                }

                layer.on_attach();
                self.layers.push(layer);
            }
            LayerCommand::Quit => {
                self.stop();
            }
        }
    }

    fn stop(&mut self) {
        while let Some(mut layer) = self.layers.pop() {
            layer.on_detach();
        }

        self.running = false;
    }
}
