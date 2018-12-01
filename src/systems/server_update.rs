use amethyst::{
    ecs::{Join, Read, ReadExpect, System, ReadStorage, WriteStorage},
    core::nalgebra::{Vector2, Vector3},
    network::*,
    core::*,
};
use std::collections::HashMap;

use {UpdateEvent, ServerEvent, TFEvent, Player, NetParams};

/// A simple system that sends a ton of messages to all connections.
/// In this case, only the server is connected.
pub struct ServerUpdate;

impl<'a> System<'a> for ServerUpdate {
    type SystemData = (
        WriteStorage<'a, NetConnection<UpdateEvent>>,
        Read<'a, Time>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Player>,
    );
    fn run(&mut self, (mut connections, time, transforms, players): Self::SystemData) {
        let mut tf_events = HashMap::new();
        // TODO: Construct maps of id -> tf_event and have that be what we send
        for (player, transform) in (&players, &transforms).join() {
            tf_events.insert(player.id, TFEvent {
                position: Vector2::new(transform.translation().x, transform.translation().y),
                velocity: Vector2::new(0.0, 0.0), // TODO
            });
        }
        let update_event = NetEvent::Custom(UpdateEvent::Server(ServerEvent {
            frame: time.frame_number(),
            tfs: tf_events,
        }));
        for mut conn in (&mut connections).join() {
            conn.send_buffer.single_write(update_event.clone());
        }
    }
}

