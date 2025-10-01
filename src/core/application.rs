use crate::core::state::{State, StateControl};
use crate::core::state_manager::StateManager;
use raylib::prelude::*;

pub struct ApplicationSpecification {
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub fps: u32,
}

pub trait Layer {
    fn update(&self, rl: &mut RaylibHandle) -> StateControl;
    fn render(&self, rl: &mut RaylibHandle, thread: &RaylibThread);
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

    pub fn clear_layers(&mut self) {
        self.layers.clear();
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.push(layer);
    }

    pub fn run(&mut self, state_manager: &mut StateManager) {
        state_manager.set_state(State::Menu, self);

        while self.running {
            if self.rl.window_should_close() {
                self.stop();
                break;
            }

            // Update
            for layer in self.layers.iter_mut() {
                match layer.update(&mut self.rl) {
                    StateControl::Continue => {}
                    StateControl::Stop => {
                        self.stop();
                        break;
                    }
                    StateControl::Change(state) => {
                        state_manager.next = Some(state);
                        state_manager.change = true;
                        break;
                    }
                }
            }

            if !self.running {
                continue;
            }

            if state_manager.change {
                if let Some(state) = state_manager.next {
                    state_manager.set_state(state, self);
                }

                state_manager.next = None;
                state_manager.change = false;
            }

            // Render
            for layer in self.layers.iter_mut() {
                layer.render(&mut self.rl, &self.thread);
            }
        }
    }

    pub fn stop(&mut self) {
        self.running = false;
    }
}
