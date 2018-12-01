use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use {Player, NetParams, GAME_HEIGHT, GAME_HEIGHT};

pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<String, String>>,
        ReadStorage<'s, NetParams>,
    );

    fn run(&mut self, (mut transforms, players, input, net_params): Self::SystemData) {
        for (player, transform) in (&players, &mut transforms).join() {
            // only move our own player
            if player.id == net_params.id {
                let movement = Vector2::new(input.axis_value("horizontal"), input.axis_value("vertical"));
                if let Some(mv_amount) = movement {
                    let scaled_amount = 1.2 * mv_amount as f32;
                    transform.translation[1] = (transform.translation[1] + scaled_amount)
                        .min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
                        .max(PADDLE_HEIGHT * 0.5);
                }
            }
        }
    }
}

