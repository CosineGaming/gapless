use amethyst::core::{Transform, Time};
use amethyst::core::math::Vector2;
use amethyst::ecs::prelude::*;
use amethyst::network::{NetConnection, NetEvent};

use crate::{network::*, Player};

pub struct NetUpdate;

impl<'s> System<'s> for NetUpdate {
    type SystemData = (
        WriteStorage<'s, NetConnection<CustomNetEvent>>,
        Read<'s, Time>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Transform>,
        ReadExpect<'s, NetParams>,
    );

    fn run(&mut self, (mut connections, time, players, transforms, net_params): Self::SystemData) {
        for (player, transform) in (&players, &transforms).join() {
            // TODO: only if the time is ripe!!!
            // Send a world-converging full-update
            if player.is_server == net_params.is_server {
                // Needed for both
                let player_pos = Vector2::new(transform.translation().x, transform.translation().y);
                let update_event = if net_params.is_server {
                    AnyUpdate::Server(ServerUpdate {
                        player_pos,
                    })
                } else {
                    AnyUpdate::Client(ClientUpdate {
                        player_pos,
                    })
                };
                let frame = time.frame_number();
                let update_event = NetEvent::Unreliable(CustomNetEvent {
                    frame,
                    from_server: net_params.is_server,
                    event: AnyEvent::Update(update_event),
                });
                for conn in (&mut connections).join() {
                    conn.send_buffer.single_write(update_event.clone());
                }
            }
        }
    }
}

