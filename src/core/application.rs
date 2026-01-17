use crate::core::action::*;
use crate::core::context::*;
use crate::core::event::*;
use crate::core::layer::*;
use crate::core::settings::*;
use crate::core::time::*;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ApplicationSpecification {
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub fps: u32,
}

pub struct Application {
    rl: RaylibHandle,
    thread: RaylibThread,
    ctx: AppContext,
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
            ctx: AppContext {
                settings: Settings::default(),
                actions: Actions::new(),
                time: Time::new(),
            },
            layers: Vec::new(),
            running: true,
        }
    }

    pub fn run(&mut self, initial_layer: Box<dyn Layer>, bindings: InputBindings) {
        self.set_initial_layer(initial_layer);
        self.set_bindings(bindings);

        if self.layers.is_empty() {
            eprintln!("Error: No initial layer set!");
            panic!();
        }

        while self.running {
            // Clear actions
            self.ctx.actions.clear();

            // Update delta time
            let delta = self.rl.get_frame_time();
            self.ctx.time.update(delta);

            // Window close check
            if self.rl.window_should_close() {
                self.stop();
                break;
            }

            // Events
            let events = collect_events(&self.rl, &self.ctx.settings.bindings);
            for event in &events {
                // Send event to current top layer - For events that need immediate processing before the update phase.
                if let Some(top_layer) = self.layers.last_mut() {
                    top_layer.on_event(&mut self.ctx, event);
                }

                // Send event to actions list - For actions handled later during update.
                match event {
                    Event::KeyPressed(key) => {
                        if let Some(action) = self.ctx.settings.bindings.key_bindings().get(key) {
                            self.ctx.actions.push(*action);
                            println!("{:?}", action);
                        }
                    }
                    Event::MousePosition(_) => {}
                }
            }

            // Update
            if let Some(top_layer) = self.layers.last_mut()
                && let Some(cmd) = top_layer.on_update(&mut self.ctx, &mut self.rl)
            {
                self.handle_layer_command(cmd);
            }

            if !self.running {
                continue;
            }

            // Render
            let mut d = self.rl.begin_drawing(&self.thread);
            d.clear_background(Color::WHITE);

            for layer in self.layers.iter_mut() {
                layer.on_render(&self.ctx, &mut d);
            }
        }
    }

    fn set_initial_layer(&mut self, mut layer: Box<dyn Layer>) {
        self.layers.clear();
        layer.on_attach(&mut self.ctx);
        self.layers.push(layer);
    }

    fn set_bindings(&mut self, bindings: InputBindings) {
        self.ctx.settings.bindings = bindings;
    }

    fn handle_layer_command(&mut self, command: LayerCommand) {
        match command {
            LayerCommand::None => {}
            LayerCommand::Push(mut layer) => {
                layer.on_attach(&mut self.ctx);
                self.layers.push(layer);
            }
            LayerCommand::Pop => {
                if let Some(mut layer) = self.layers.pop() {
                    layer.on_detach(&mut self.ctx);
                }

                if self.layers.is_empty() {
                    self.stop();
                }
            }
            LayerCommand::Replace(mut layer) => {
                for mut old_layer in self.layers.drain(..) {
                    old_layer.on_detach(&mut self.ctx);
                }

                layer.on_attach(&mut self.ctx);
                self.layers.push(layer);
            }
            LayerCommand::Quit => {
                self.stop();
            }
        }
    }

    fn stop(&mut self) {
        while let Some(mut layer) = self.layers.pop() {
            layer.on_detach(&mut self.ctx);
        }

        self.running = false;
    }
}
