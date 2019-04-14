use amethyst::{
    ecs::{Join, ReaderId, System, ReadStorage, WriteStorage, ReadExpect},
    network::*,
    core::Transform,
    core::math::Vector3,
};

use crate::{Player, network::*};

/// A simple system that receives a ton of network events.
pub struct NetReceive {
    pub reader: Option<ReaderId<NetEvent<CustomNetEvent>>>,
}

impl NetReceive {
    pub fn new() -> Self {
        NetReceive { reader: None }
    }
}

impl<'a> System<'a> for NetReceive {
    type SystemData = (
        WriteStorage<'a, NetConnection<CustomNetEvent>>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Player>,
        ReadExpect<'a, NetParams>,
    );
    fn run(&mut self, (mut connections, mut transforms, players, net_params): Self::SystemData) {
        for (conn,) in (&mut connections,).join() {
            if self.reader.is_none() {
                self.reader = Some(conn.receive_buffer.register_reader());
            }
            let mut update_recent: Option<CustomNetEvent> = None;
            for ev in conn.receive_buffer.read(self.reader.as_mut().unwrap()) {
                match ev {
                    NetEvent::Unreliable(event) => {
                        // All our unreliable events are updates, we only need the most recent one
                        if update_recent == None || event.frame > update_recent.as_ref().unwrap().frame {
                            update_recent = Some(event.clone());
                        }
                    },
                    NetEvent::Reliable(event) => {
                        match &event.event {
                            AnyEvent::Input(e) => {
                                for (player, transform) in (&players, &mut transforms).join() {
                                    if player.is_server == event.from_server {
                                        let movement = get_movement(e.clone()); // TODO: there should def be a way to not have to clone this
                                        transform.prepend_translation(movement);
                                    }
                                }
                            }
                            _ => panic!("non-reliable InputEvent found")
                        }
                    },
                    _ => panic!("unexpected NetEvent unhandled!"),
                }
            }
            if let Some(update_recent) = update_recent {
                if let AnyEvent::Update(update_recent) = update_recent.event {
                    for (player, transform) in (&players, &mut transforms).join() {
                        if net_params.is_server != player.is_server {
                            let pos = match update_recent {
                                AnyUpdate::Server(ref e) if !net_params.is_server => e.player_pos,
                                AnyUpdate::Client(ref e) if net_params.is_server => e.player_pos,
                                _ => continue, // Our own update
                            };
                            transform.set_translation_xyz(pos.x, pos.y, 0.0);
                        }
                    }
                } else { panic!("expected update event in recent, but it wasn't") }
            }
        }
    }
}

/// TODO: figure out how best to break this into another file
fn get_movement(input: InputEvent) -> Vector3<f32> {
    // only move our own player
    let mut movement = Vector3::zeros();
    if input.right {
        movement.x += 1.0;
    }
    if input.left {
        movement.x -= 1.0;
    }
    if input.up {
        movement.y += 1.0;
    }
    if input.down {
        movement.y -= 1.0;
    }
    if movement != Vector3::zeros() {
        movement = movement.normalize() * 2.5;
    }
    movement
    // TODO: Framerate dependent????
    // TODO: Edges of screen / collisions / etc / make a game lol
    // let scaled_amount = 1.2 * mv_amount as f32;
    // transform.translation[1] = (transform.translation[1] + scaled_amount)
    //     .min(GAME_HEIGHT) // get height and adjust for it
    //     .max(0); // same
}

