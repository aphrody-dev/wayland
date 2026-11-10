pub mod hyperv;

use crate::wire::{WaylandMessage, WireError};
use std::collections::VecDeque;

pub trait WaylandTransport: Send + Sync {
    fn send(&mut self, msg: &WaylandMessage) -> Result<(), WireError>;
    fn receive(&mut self) -> Result<Option<WaylandMessage>, WireError>;
    fn is_connected(&self) -> bool;
}

pub struct LoopbackTransport {
    pub client_to_server: VecDeque<WaylandMessage>,
    pub server_to_client: VecDeque<WaylandMessage>,
    pub connected: bool,
}

impl LoopbackTransport {
    pub fn new() -> Self {
        Self {
            client_to_server: VecDeque::new(),
            server_to_client: VecDeque::new(),
            connected: true,
        }
    }

    pub fn client_send(&mut self, msg: WaylandMessage) {
        self.client_to_server.push_back(msg);
    }

    pub fn server_receive(&mut self) -> Option<WaylandMessage> {
        self.client_to_server.pop_front()
    }

    pub fn server_send(&mut self, msg: WaylandMessage) {
        self.server_to_client.push_back(msg);
    }

    pub fn client_receive(&mut self) -> Option<WaylandMessage> {
        self.server_to_client.pop_front()
    }
}
