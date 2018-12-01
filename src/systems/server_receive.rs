use amethyst::{
    ecs::prelude::*,
    network::*,
    core::nalgebra::{Vector3},
    core::Transform,
    shrev::EventChannel,
};

use {Player, UpdateEvent, ClientEvent};

/// A simple system that receives a ton of network events.
pub struct ServerReceive {
    pub reader: Option<ReaderId<NetEvent<UpdateEvent>>>,
}

impl ServerReceive {
    pub fn new() -> Self {
        ServerReceive { reader: None }
    }
}

impl<'a> System<'a> for ServerReceive {
    type SystemData = (
        WriteStorage<'a, NetConnection<UpdateEvent>>,
        Write<'a, EventChannel<ClientEvent>>,
    );
    fn run(&mut self, (mut connections, mut channel): Self::SystemData) {
        for (mut conn,) in (&mut connections,).join() {
            if self.reader.is_none() {
                self.reader = Some(conn.receive_buffer.register_reader());
            }
            for ev in conn.receive_buffer.read(self.reader.as_mut().unwrap()) {
                if let NetEvent::Custom(event) = ev {
                    if let UpdateEvent::Client(client_event) = event {
                        channel.single_write(client_event.clone());
                    }
                }
            }
        }
    }
}

