use crate::app::action::*;
use crate::app::layers::menu::MenuLayer;
use crate::app::layers::pause::PauseLayer;
use crate::core::context::*;
use crate::core::event::*;
use crate::core::layer::*;
use crate::net::client::*;
use crate::net::protocol::message::*;
use crate::net::server::*;
use crate::net::transport::endpoint::*;
use crate::net::transport::loopback::*;
use raylib::prelude::*;

pub struct GameLayer {
    client: Client<LoopBackClientEndpoint>,
    server: Server<LoopBackServerEndpoint>,
}

impl Default for GameLayer {
    fn default() -> Self {
        let (client_endpoint, server_endpoint) = loopback();
        let client = Client::new(client_endpoint);
        let server = Server::new(server_endpoint);

        Self { client, server }
    }
}

impl Layer<Action> for GameLayer {
    fn on_event(&mut self, ctx: &mut AppContext<Action>, event: &Event) {}

    fn on_update(
        &mut self,
        ctx: &mut AppContext<Action>,
        rl: &mut RaylibHandle,
    ) -> Option<LayerCommand<Action>> {
        //Client/Server stuff
        self.client.add_time(ctx.time.delta());
        while self.client.get_accumulator() >= self.server.fixed_dt() {
            self.server.tick();
            self.client.subtract_time(self.server.fixed_dt());
        }

        self.client.get_server_messages();

        // Layer stuff
        if ctx.actions.take(Action::Confirm) {
            return Some(LayerCommand::Replace(Box::new(MenuLayer)));
        }

        if ctx.actions.take(Action::Pause) {
            return Some(LayerCommand::Push(Box::new(PauseLayer)));
        }

        if ctx.actions.take(Action::Quit) {
            return Some(LayerCommand::Quit);
        }

        None
    }

    fn on_render(&mut self, ctx: &AppContext<Action>, d: &mut RaylibDrawHandle) {
        d.draw_text("This is the game layer!", 12, 12, 20, Color::BLACK);

        if let Some(snapshot) = &self.client.server_world_snapshot() {
            d.draw_text(
                &format!("Server tick: {}", snapshot.snapshot_tick()),
                12,
                40,
                20,
                Color::BLACK,
            );

            for entity in snapshot.entity_positions() {
                d.draw_circle(entity.x as i32, entity.y as i32, 10.0, Color::BLUE);
            }
        } else {
            d.draw_text("Waiting for snapshot...", 12, 60, 20, Color::DARKGRAY);
        }
    }

    fn on_attach(&mut self, ctx: &mut AppContext<Action>) {
        println!("Attaching game layer...");

        self.client
            .mut_endpoint()
            .send(ClientMessage::Connect)
            .unwrap();
    }

    fn on_detach(&mut self, ctx: &mut AppContext<Action>) {
        println!("Detaching game layer...");

        self.client
            .mut_endpoint()
            .send(ClientMessage::Disconnect)
            .unwrap();

        self.server.tick();
    }
}
