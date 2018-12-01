use amethyst::core::Transform;
use amethyst::core::nalgebra::Vector3;
use amethyst::ecs::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use {Player, NetParams, GAME_WIDTH, GAME_HEIGHT};

pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<String, String>>,
        ReadExpect<'s, NetParams>,
    );

    fn run(&mut self, (mut transforms, players, input, net_params): Self::SystemData) {
        for (player, transform) in (&players, &mut transforms).join() {
            // only move our own player
            if player.id == net_params.id {
                let movement = Vector3::new(input.axis_value("horizontal").unwrap() as f32, input.axis_value("vertical").unwrap() as f32, 0.0);
                let movement = movement * 2.0;
                transform.move_global(movement);
                // TODO: Framerate dependent????
                // let scaled_amount = 1.2 * mv_amount as f32;
                // transform.translation[1] = (transform.translation[1] + scaled_amount)
                //     .min(GAME_HEIGHT) // get height and adjust for it
                //     .max(0); // same
            }
        }
    }
}

