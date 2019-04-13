use amethyst::{
    ecs::prelude::*,
    network::*,
    shrev::EventChannel,
};

use crate::UpdateEvent;

/// A simple system that receives a ton of network events.
pub struct NetReceive {
    pub reader: Option<ReaderId<NetEvent<UpdateEvent>>>,
}

impl NetReceive {
    pub fn new() -> Self {
        NetReceive { reader: None }
    }
}

impl<'a> System<'a> for NetReceive {
    type SystemData = (
        WriteStorage<'a, NetConnection<UpdateEvent>>,
        Write<'a, EventChannel<UpdateEvent>>,
    );
    fn run(&mut self, (mut connections, mut events): Self::SystemData) {
        for (conn,) in (&mut connections,).join() {
	        if self.reader.is_none() {
		        self.reader = Some(conn.receive_buffer.register_reader());
	        }
            let mut tf_recent: Option<UpdateEvent> = None;
            for ev in conn.receive_buffer.read(self.reader.as_mut().unwrap()) {
                match ev {
                    NetEvent::Unreliable(event) => {
		                // All our unreliable events are updates, we only need the most recent one
                        if tf_recent == None || tf_recent.as_ref().unwrap().frame < event.frame {
                            tf_recent = Some(event.clone());
                        }
                    },
                    NetEvent::Reliable(event) => {
                        events.single_write(event.clone());
                    },
                    _ => panic!("unexpected NetEvent unhandled!"),
                }
            }
            if let Some(event) = tf_recent {
                events.single_write(event.clone());
            }
        }
    }
}

