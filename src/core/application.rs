use raylib::prelude::*;

pub trait Layer {
    fn on_update(&mut self, rl: &mut RaylibHandle) -> LayerCommand;
    fn on_render(&mut self, d: &mut RaylibDrawHandle);
}

pub enum LayerCommand {
    None,
    Push(Box<dyn Layer>),
    Pop,
    Replace(Box<dyn Layer>),
    Quit,
}

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

    pub fn set_initial_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.clear();
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

            // Update
            if let Some(top_layer) = self.layers.last_mut() {
                match top_layer.on_update(&mut self.rl) {
                    LayerCommand::None => {}
                    LayerCommand::Push(layer) => {
                        self.layers.push(layer);
                    }
                    LayerCommand::Pop => {
                        self.layers.pop();

                        if self.layers.is_empty() {
                            self.stop();
                        }
                    }
                    LayerCommand::Replace(layer) => {
                        self.layers.clear();
                        self.layers.push(layer);
                    }
                    LayerCommand::Quit => {
                        self.stop();
                    }
                }
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

    pub fn stop(&mut self) {
        self.running = false;
    }
}
