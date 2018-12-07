use amethyst::{
    ecs::{Join, ReaderId, System, ReadStorage, WriteStorage, ReadExpect},
    network::*,
    core::nalgebra::{Vector3},
    core::Transform,
};

use {Player, UpdateEvent, NetParams};

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
        for (mut conn,) in (&mut connections,).join() {
            if self.reader.is_none() {
                self.reader = Some(conn.receive_buffer.register_reader());
            }
            let mut iter = conn.receive_buffer.read(self.reader.as_mut().unwrap())
                .filter_map(|ev| {
                    // TODO: match
                    if let NetEvent::Custom(event) = ev {
                        Some(event)
                    } else {
                        None
                    }
                });
            if let Some(first) = iter.next() {
                let recent = iter.fold(first, |acc, ev| {
                    if acc.frame > ev.frame { acc } else { ev }
                });
                for (player, mut transform) in (&players, &mut transforms).join() {
                    if net_params.is_server != player.is_server {
                        let pos = recent.tf.position;
                        transform.set_xyz(pos.x, pos.y, 0.0);
                    }
                }
            }
        }
    }
}

