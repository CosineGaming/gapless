use amethyst::ecs::prelude::*;
use amethyst::ecs::Join;
use amethyst::renderer::SpriteRender;
use amethyst::core::Transform;
use amethyst::core::Time;

use crate::{AbilityComp};

pub struct AbilitySystem;

impl AbilitySystem {
    pub fn new() -> Self {
        AbilitySystem {}
    }
}

fn count_to_total(count: f32) -> f32 {
	count.powi(5)
}

impl<'s> System<'s> for AbilitySystem {
    type SystemData = (
        WriteStorage<'s, AbilityComp>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Transform>,
        Entities<'s>,
        ReadExpect<'s, Time>,
    );

    fn run(&mut self, (mut abilities, mut sprites, mut transforms, entities, time): Self::SystemData) {
        for (ability, sprite, entity) in (&mut abilities, &mut sprites, &*entities).join() {
	        let state = &mut ability.state;
            state.count += state.direction as f32 * time.delta_seconds();
            if state.direction == 1 && state.count >= state.freq
            	|| state.direction == -1 && state.count <= 0. {
	            state.direction = -state.direction;
	            // Don't go out of bounds. TODO: Don't duplicate with above
	            state.count += state.direction as f32 * time.delta_seconds();
            }
            sprite.sprite_number = (state.count * ability.frames as f32 / state.freq) as usize;
            println!("{}", count_to_total(state.count));
	        let player_tf = transforms.get(ability.target).unwrap();
	        *transforms.get_mut(entity).unwrap() = player_tf.clone();
        }
    }
}


