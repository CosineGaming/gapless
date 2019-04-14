use amethyst::{
    ecs::{Join, ReaderId, System, ReadStorage, WriteStorage, ReadExpect},
    network::*,
    core::Transform,
};

use crate::{Player, UpdateEvent, NetParams};

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
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Player>,
        ReadExpect<'a, NetParams>,
    );
    fn run(&mut self, (mut connections, mut transforms, players, net_params): Self::SystemData) {
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
	                    // Handle immediately
	                    match event {
		                    _ => panic!("no reliable events expected yet"),
	                    }
                    },
                    _ => panic!("unexpected NetEvent unhandled!"),
                }
            }
            if let Some(tf_recent) = tf_recent {
	            for (player, transform) in (&players, &mut transforms).join() {
	                if net_params.is_server != player.is_server {
	                    let pos = tf_recent.tf.position;
	                    transform.set_translation_xyz(pos.x, pos.y, 0.0);
	                }
	            }
            }
        }
    }
}

