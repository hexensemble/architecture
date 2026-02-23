use crate::net::protocol::message::*;
use crate::net::transport::endpoint::*;
use crate::net::transport::error::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type MessageQueue<T> = Rc<RefCell<VecDeque<T>>>;

pub fn loopback() -> (LoopBackClientEndpoint, LoopBackServerEndpoint) {
    let client_to_server_queue: MessageQueue<ClientMessage> =
        Rc::new(RefCell::new(VecDeque::new()));

    let server_to_client_queue: MessageQueue<ServerMessage> =
        Rc::new(RefCell::new(VecDeque::new()));

    let client_endpoint = LoopBackClientEndpoint {
        outgoing: Rc::clone(&client_to_server_queue),
        incoming: Rc::clone(&server_to_client_queue),
    };

    let server_endpoint = LoopBackServerEndpoint {
        outgoing: Rc::clone(&server_to_client_queue),
        incoming: Rc::clone(&client_to_server_queue),
    };

    (client_endpoint, server_endpoint)
}

pub struct LoopBackClientEndpoint {
    outgoing: MessageQueue<ClientMessage>,
    incoming: MessageQueue<ServerMessage>,
}

impl ClientEndpoint for LoopBackClientEndpoint {
    fn send(&mut self, msg: ClientMessage) -> Result<(), TransportError> {
        self.outgoing.borrow_mut().push_back(msg);
        Ok(())
    }

    fn recv(&mut self) -> Option<ServerMessage> {
        self.incoming.borrow_mut().pop_front()
    }
}

pub struct LoopBackServerEndpoint {
    outgoing: MessageQueue<ServerMessage>,
    incoming: MessageQueue<ClientMessage>,
}

impl ServerEndpoint for LoopBackServerEndpoint {
    fn send(&mut self, msg: ServerMessage) -> Result<(), TransportError> {
        self.outgoing.borrow_mut().push_back(msg);
        Ok(())
    }

    fn recv(&mut self) -> Option<ClientMessage> {
        self.incoming.borrow_mut().pop_front()
    }
}
