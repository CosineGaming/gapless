use amethyst::core::Transform;
use amethyst::core::nalgebra::Vector3;
use amethyst::ecs::prelude::*;
use amethyst::ecs::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage, ReaderId, Resources};
use amethyst::input::InputHandler;
use amethyst::shrev::EventChannel;

use {UpdateEvent, ClientEvent, Player, NetParams, GAME_WIDTH, GAME_HEIGHT};

pub struct PlayerSystem {
    reader: Option<ReaderId<ClientEvent>>,
}

impl PlayerSystem {
    pub fn new() -> Self {
        PlayerSystem {
            reader: None
        }
    }
}

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        Read<'s, EventChannel<ClientEvent>>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        ReadExpect<'s, NetParams>,
    );

    fn run(&mut self, (channel, mut transforms, players, net_params): Self::SystemData) {
        // TODO: Turn this two-layer loop faster?
        for event in channel.read(self.reader.as_mut().unwrap()) {
            for (player, transform) in (&players, &mut transforms).join() {
                // only move our own player
                if player.id == event.id {
                    let mut movement = Vector3::new(0.0, 0.0, 0.0);
                    if event.right {
                        movement.x += 1.0;
                    }
                    if event.left {
                        movement.x -= 1.0;
                    }
                    if event.up {
                        movement.y += 1.0;
                    }
                    if event.down {
                        movement.y -= 1.0;
                    }
                    let movement = movement * 2.0;
                    transform.move_global(movement);
                    // TODO: Framerate dependent????
                    // TODO: Edges of screen / collisions / etc / make a game lol
                    // let scaled_amount = 1.2 * mv_amount as f32;
                    // transform.translation[1] = (transform.translation[1] + scaled_amount)
                    //     .min(GAME_HEIGHT) // get height and adjust for it
                    //     .max(0); // same
                }
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        // IMPORTANT: You need to setup your system data BEFORE you try to fetch the resource. Especially if you plan use `Default` to create your resource.
        Self::SystemData::setup(res);
        self.reader = Some(res.fetch_mut::<EventChannel<ClientEvent>>().register_reader());
    }
}

