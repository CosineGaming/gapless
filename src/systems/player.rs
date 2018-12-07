use amethyst::core::Transform;
use amethyst::core::nalgebra::Vector3;
use amethyst::ecs::prelude::*;
use amethyst::ecs::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage, ReaderId, Resources};
use amethyst::input::InputHandler;
use amethyst::shrev::EventChannel;

use {UpdateEvent, Player, NetParams, GAME_WIDTH, GAME_HEIGHT};

pub struct PlayerSystem;

impl PlayerSystem {
    pub fn new() -> Self {
        PlayerSystem {}
    }
}

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
            if player.is_server == net_params.is_server {
                let mut movement = Vector3::new(0.0, 0.0, 0.0);
                if input.action_is_down("right").unwrap() {
                    movement.x += 1.0;
                }
                if input.action_is_down("left").unwrap() {
                    movement.x -= 1.0;
                }
                if input.action_is_down("up").unwrap() {
                    movement.y += 1.0;
                }
                if input.action_is_down("down").unwrap() {
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

