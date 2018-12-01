use amethyst::{
    ecs::{Join, ReaderId, System, ReadStorage, WriteStorage},
    network::*,
    core::nalgebra::{Vector3},
    core::Transform,
};

use {Player, UpdateEvent, ServerEvent};

/// A simple system that receives a ton of network events.
pub struct ClientReceive {
    pub reader: Option<ReaderId<NetEvent<UpdateEvent>>>,
}

impl ClientReceive {
    pub fn new() -> Self {
        ClientReceive { reader: None }
    }
}

impl<'a> System<'a> for ClientReceive {
    type SystemData = (
        WriteStorage<'a, NetConnection<UpdateEvent>>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Player>,
    );
    fn run(&mut self, (mut connections, mut transforms, players): Self::SystemData) {
        for (mut conn,) in (&mut connections,).join() {
            if self.reader.is_none() {
                self.reader = Some(conn.receive_buffer.register_reader());
            }
            let mut iter = conn.receive_buffer.read(self.reader.as_mut().unwrap())
                .filter_map(|ev| {
                    // TODO: match
                    if let NetEvent::Custom(event) = ev {
                        if let UpdateEvent::Server(server_event) = event {
                            Some(server_event)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                });
            if let Some(first) = iter.next() {
                let recent = iter.fold(first, |acc, ev| {
                    if acc.frame > ev.frame { acc } else { ev }
                });
                for (player, mut transform) in (&players, &mut transforms).join() {
                    if let Some(tf) = recent.tfs.get(&player.id) {
                        let pos = tf.position;
                        transform.set_xyz(pos.x, pos.y, 0.0);
                    }
                }
            }
        }
    }
}

