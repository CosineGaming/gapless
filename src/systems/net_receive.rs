use amethyst::{
    ecs::prelude::*,
    network::*,
    core::{Transform, Time},
};
use std::collections::HashMap;

use crate::{Player, network::*, components::ordered_input::*};

static DEFAULT_LAG: u64 = 6;

/// A simple system that receives a ton of network events.
pub struct NetReceive {
    pub reader: Option<ReaderId<NetEvent<CustomNetEvent>>>,
    // Maps frame to input frame
    input_buffer: HashMap<u64, OwnedInput>,
    // Because updates, like inputs are lagged, we need to buffer it as well
    update_buffer: HashMap<u64, AnyUpdate>,
    // In frames. ~100ms
    lag_frames: u64,
}
impl Default for NetReceive {
    fn default() -> Self {
        Self {
            reader: None,
            input_buffer: HashMap::new(),
            update_buffer: HashMap::new(),
            lag_frames: DEFAULT_LAG,
        }
    }
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
        // TODO: since i don't anticipate multiple updates arriving
        // simultaneously, this loop logic is somewhat extraneous, or at least
        // run "maximizing" on the buffer instead of the channel
        for (conn,) in (&mut connections,).join() {
            let mut update_recent: Option<CustomNetEvent> = None;
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
                            }
                            _ => panic!("reliable non-InputEvent found")
                        }
                    },
                    _ => panic!("unexpected NetEvent unhandled!"),
                }
            }
            // Now we take the most recent update and buffer it
            if let Some(update_recent) = update_recent {
                if let AnyEvent::Update(any_update) = update_recent.event {
                    self.update_buffer.insert(update_recent.frame, any_update);
                } else { panic!("expected update event in recent, but it wasn't") }
            }
            // Actually use our input and update buffers
            let frame = time.frame_number() - net_params.first_frame;
            println!("frame {}", frame);
            if frame > self.lag_frames {
                let next_frame_number = frame - self.lag_frames;

                // Input buffer
                let maybe_next_frame = self.input_buffer.remove(&next_frame_number);
                match maybe_next_frame {
                    Some(input) => out_input.single_write(input),
                    // Save this frame for later when we hope to get it - increases
                    // latency, but reduces popping(?).... CHECK
                    // CHECK: is there connection frame-lag?
                    // TODO: explain magic if
                    None => if next_frame_number > 5 {
                        self.lag_frames += 1
                    }
                }
                println!("{} (waiting for {})", self.lag_frames, next_frame_number);

                // Update buffer
                let maybe_update = self.update_buffer.remove(&next_frame_number);
                match maybe_update {
                    Some(update) => {
                        // Update the state to that described in the update
                        for (player, transform) in (&players, &mut transforms).join() {
                            if net_params.is_server != player.is_server {
                                let pos = match update {
                                    AnyUpdate::Server(ref e) if !net_params.is_server => e.player_pos,
                                    AnyUpdate::Client(ref e) if net_params.is_server => e.player_pos,
                                    _ => panic!("received our own update"),
                                };
                                transform.set_translation_xyz(pos.x, pos.y, 0.0);
                            }
                        }
                    }
                    None => (), // No update, no problem
                }
            }
        }
    }
}

