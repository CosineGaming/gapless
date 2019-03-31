use amethyst::core::{Transform, Time};
use amethyst::core::math::Vector2;
use amethyst::ecs::prelude::*;
use amethyst::network::{NetConnection, NetEvent};

use crate::{UpdateEvent, Player, NetParams, TFEvent};

pub struct NetUpdate;

impl<'s> System<'s> for NetUpdate {
    type SystemData = (
        WriteStorage<'s, NetConnection<UpdateEvent>>,
        Read<'s, Time>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Transform>,
        ReadExpect<'s, NetParams>,
    );

    fn run(&mut self, (mut connections, time, players, transforms, net_params): Self::SystemData) {
        for (player, transform) in (&players, &transforms).join() {
            // Send your own player's transform
            if player.is_server == net_params.is_server {
                let update_event = UpdateEvent {
                    frame: time.frame_number(),
                    tf: TFEvent {
                        position: Vector2::new(transform.translation().x, transform.translation().y), // TODO: implement
                        velocity: Vector2::new(0.0, 0.0), // TODO: implement
                    },
                };
                let update_event = NetEvent::Custom(update_event);
                for mut conn in (&mut connections).join() {
                    conn.send_buffer.single_write(update_event.clone());
                }
            }
        }
    }
}

