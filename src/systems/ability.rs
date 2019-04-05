use amethyst::ecs::prelude::*;
use amethyst::ecs::Join;
use amethyst::renderer::SpriteRender;
use amethyst::core::Transform;
use amethyst::core::Time;

use crate::Ability;

pub struct AbilitySystem;

impl AbilitySystem {
    pub fn new() -> Self {
        AbilitySystem {}
    }
}

impl<'s> System<'s> for AbilitySystem {
    type SystemData = (
        WriteStorage<'s, Ability>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Transform>,
        Entities<'s>,
        ReadExpect<'s, Time>,
    );

    fn run(&mut self, (mut abilities, mut sprites, mut transforms, entities, time): Self::SystemData) {
        for (ability, sprite, entity) in (&mut abilities, &mut sprites, &*entities).join() {
            ability.count += ability.direction as f32 * time.delta_seconds();
            if ability.direction == 1 && ability.count >= ability.freq
            	|| ability.direction == -1 && ability.count <= 0. {
	            ability.direction = -ability.direction;
	            // Don't go out of bounds. TODO: Don't duplicate with above
	            ability.count += ability.direction as f32 * time.delta_seconds();
            }
            sprite.sprite_number = (ability.count * ability.frames as f32 / ability.freq) as usize;
            println!("{}", ability.count);
	        let player_tf = transforms.get(ability.target).unwrap();
	        *transforms.get_mut(entity).unwrap() = player_tf.clone();
        }
    }
}


