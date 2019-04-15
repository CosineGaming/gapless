use amethyst::core::{Transform, Time};
use amethyst::core::math::Vector2;
use amethyst::ecs::prelude::*;
use amethyst::network::{NetConnection, NetEvent};

use crate::{network::*, Player};

static DRIFT_FRAMES: usize = 60;
static PACKET_LOSS: f32 = 0.9;
static UPDATE_FREQ: u64 = (DRIFT_FRAMES as f32 * PACKET_LOSS) as u64;

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
        // Update is only meant to correct drift, not to convey data;
        // Only update occasionally
        if time.frame_number() % UPDATE_FREQ == 0 {
            for (player, transform) in (&players, &transforms).join() {
                // TODO: only if the time is ripe!!!
                // Send a world-converging full-update
                if player.is_server == net_params.is_server {
                    // Needed for both
                    let player_pos = Vector2::new(transform.translation().x, transform.translation().y);
                    let any_update = if net_params.is_server {
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
                        event: AnyEvent::Update(any_update),
                    });
                    for conn in (&mut connections).join() {
                        conn.send_buffer.single_write(update_event.clone());
                    }
                }
            }
        }
    }
}

