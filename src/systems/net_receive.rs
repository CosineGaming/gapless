use amethyst::{
    ecs::prelude::*,
    network::*,
    core::{Transform, Time},
};
use std::collections::HashMap;

use crate::{Player, network::*, components::ordered_input::*};

// In frames. ~100ms
static INPUT_LAG: u64 = 6;

/// A simple system that receives a ton of network events.
#[derive(Default)]
pub struct NetReceive {
    pub reader: Option<ReaderId<NetEvent<CustomNetEvent>>>,
    // Maps frame to input frame
    input_buffer: HashMap<u64, OwnedInput>,
}

impl<'a> System<'a> for NetReceive {
    type SystemData = (
        WriteStorage<'a, NetConnection<CustomNetEvent>>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Player>,
        Write<'a, OrderedInput>,
        WriteExpect<'a, NetParams>,
        Read<'a, Time>,
    );
    fn run(&mut self, (mut connections, mut transforms, players, mut out_input, mut net_params, time): Self::SystemData) {
        let mut update_recent: Option<CustomNetEvent> = None;
        for (conn,) in (&mut connections,).join() {
            if self.reader.is_none() {
                self.reader = Some(conn.receive_buffer.register_reader());
                net_params.first_frame = time.frame_number();
            }
            for ev in conn.receive_buffer.read(self.reader.as_mut().unwrap()) {
                println!("{:?}", ev);
                // Nab the event
                match ev {
                    NetEvent::Unreliable(event) => {
                        // All our unreliable events are updates, we only need the most recent one
                        if update_recent == None || event.frame > update_recent.as_ref().unwrap().frame {
                            update_recent = Some(event.clone());
                        }
                    },
                    NetEvent::Reliable(event) => {
                        // Our reliable events are input data, which we'll store in the buffer for later
                        match &event.event {
                            AnyEvent::Input(e) => {
                                self.input_buffer.insert(event.frame, OwnedInput {
                                    input: e.clone(),
                                    is_server: event.from_server,
                                });
                                println!("recieved {:?} for frame {}", e, event.frame);
                            }
                            _ => panic!("non-reliable InputEvent found")
                        }
                    },
                    _ => panic!("unexpected NetEvent unhandled!"),
                }
            }
        }
        // Now we take the most recent update and update the state
        if let Some(update_recent) = update_recent {
            if let AnyEvent::Update(update_recent) = update_recent.event {
                // Update the state to that described in the update
                for (player, transform) in (&players, &mut transforms).join() {
                    if net_params.is_server != player.is_server {
                        let pos = match update_recent {
                            AnyUpdate::Server(ref e) if !net_params.is_server => e.player_pos,
                            AnyUpdate::Client(ref e) if net_params.is_server => e.player_pos,
                            _ => panic!("received our own update"),
                        };
                        transform.set_translation_xyz(pos.x, pos.y, 0.0);
                    }
                }
            } else { panic!("expected update event in recent, but it wasn't") }
        }
        // Deal with input buffer
        let frame = time.frame_number() - net_params.first_frame;
        if frame > INPUT_LAG {
            let next_frame_number = frame - INPUT_LAG;
            let maybe_next_frame = self.input_buffer.remove(&next_frame_number);
            match maybe_next_frame {
                Some(input) => out_input.single_write(input),
                None => println!("ERROR: unimplemented: if we don't have a needed frame"),
            }
        }
    }
}

