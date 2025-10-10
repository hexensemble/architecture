use raylib::prelude::*;

pub trait Layer {
    fn update(&mut self, rl: &mut RaylibHandle) -> LayerControl;
    fn render(&mut self, d: &mut RaylibDrawHandle);
}

pub struct LayerControl {
    pub change_layer: bool,
    pub next_layer: Option<Box<dyn Layer>>,
    pub running: bool,
}

impl LayerControl {
    pub fn continue_running() -> Self {
        Self {
            change_layer: false,
            next_layer: None,
            running: true,
        }
    }

    pub fn change_layer(next_layer: Option<Box<dyn Layer>>) -> Self {
        Self {
            change_layer: true,
            next_layer,
            running: true,
        }
    }

    pub fn quit() -> Self {
        Self {
            change_layer: false,
            next_layer: None,
            running: false,
        }
    }
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
        self.clear_layers();
        self.push_layer(layer);
    }

    pub fn clear_layers(&mut self) {
        self.layers.clear();
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
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
            for layer in self.layers.iter_mut() {
                let layer_control = layer.update(&mut self.rl);

                if layer_control.change_layer {
                    self.clear_layers();

                    if let Some(layer) = layer_control.next_layer {
                        self.push_layer(layer);
                    }

                    break;
                }

                if !layer_control.running {
                    self.stop();

                    break;
                }
            }

            if !self.running {
                continue;
            }

            // Render
            let mut d = self.rl.begin_drawing(&self.thread);
            d.clear_background(Color::WHITE);

            for layer in self.layers.iter_mut() {
                layer.render(&mut d);
            }
        }
    }

    pub fn stop(&mut self) {
        self.running = false;
    }
}
