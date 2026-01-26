use crate::core::action::*;
use crate::core::bindings::*;
use crate::core::context::*;
use crate::core::event::*;
use crate::core::layer::*;
use crate::core::settings::*;
use crate::core::time::*;
use raylib::prelude::*;

pub struct Application<A: ActionType> {
    rl: RaylibHandle,
    thread: RaylibThread,
    ctx: AppContext<A>,
    layers: Vec<Box<dyn Layer<A>>>,
    running: bool,
}

impl<A: ActionType> Application<A> {
    pub fn new(settings: Settings<A>) -> Self {
        let (mut rl, thread) = raylib::init()
            .size(settings.width, settings.height)
            .title(&settings.title)
            .build();

        rl.set_target_fps(settings.fps);

        let bindings = InputBindings::new(&settings.serialized_bindings);

        Application {
            rl,
            thread,
            ctx: AppContext {
                settings,
                bindings,
                actions: Actions::new(),
                time: Time::new(),
            },
            layers: Vec::new(),
            running: true,
        }
    }

    pub fn run(
        &mut self,
        initial_layer: Box<dyn Layer<A>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.set_initial_layer(initial_layer);

        if self.layers.is_empty() {
            return Err("No initial layer set.".into());
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
            let events = collect_events(&self.rl, &self.ctx.bindings);
            for event in &events {
                // Send event to current top layer - For events that need immediate processing before the update phase.
                if let Some(top_layer) = self.layers.last_mut() {
                    top_layer.on_event(&mut self.ctx, event);
                }

                // Send event to actions list - For actions handled later during update.
                match event {
                    Event::KeyPressed(key) => {
                        if let Some(action) = self.ctx.bindings.key_bindings().get(key) {
                            self.ctx.actions.push(*action);
                            println!("{:?}", action);
                        }
                    }
                    Event::PadPressed(_gamepad_id, pad) => {
                        if let Some(action) = self.ctx.bindings.pad_bindings().get(pad) {
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

        Ok(())
    }

    fn set_initial_layer(&mut self, mut layer: Box<dyn Layer<A>>) {
        self.layers.clear();
        layer.on_attach(&mut self.ctx);
        self.layers.push(layer);
    }

    fn handle_layer_command(&mut self, command: LayerCommand<A>) {
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
