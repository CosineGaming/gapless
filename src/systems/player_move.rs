use amethyst::{
    ecs::prelude::*,
    core::Transform,
    core::math::Vector3,
};

use crate::{
    Player,
    components::ordered_input::*,
};

/// Uses the collected movement data from network and local to move players
#[derive(Default)]
pub struct MoveSystem {
    pub reader: Option<ReaderId<OwnedInput>>,
}

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        Read<'s, OrderedInput>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
    );
    fn setup(&mut self, res: &mut amethyst::ecs::Resources) {
        Self::SystemData::setup(res);
        self.reader = Some(res.fetch_mut::<OrderedInput>().register_reader());
    }

    fn run(&mut self, (input, players, mut transforms): Self::SystemData) {
        for input in input.read(self.reader.as_mut().unwrap()) {
            for (player, transform) in (&players, &mut transforms).join() {
                // only move our own player
                if input.is_server == player.is_server {
                    let input = &input.input;
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
                    transform.prepend_translation(movement);
                }
            }
        }
    }
    // TODO: Framerate dependent????
    // TODO: Edges of screen / collisions / etc / make a game lol
    // let scaled_amount = 1.2 * mv_amount as f32;
    // transform.translation[1] = (transform.translation[1] + scaled_amount)
    //     .min(GAME_HEIGHT) // get height and adjust for it
    //     .max(0); // same
}
