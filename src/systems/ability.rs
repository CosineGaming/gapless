use amethyst::ecs::prelude::*;
use amethyst::ecs::Join;
use amethyst::renderer::SpriteRender;
use amethyst::core::Transform;

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
    );

    fn run(&mut self, (mut abilities, mut sprites, mut transforms, entities): Self::SystemData) {
        for (ability, sprite, entity) in (&mut abilities, &mut sprites, &*entities).join() {
            if ability.direction == 1 && ability.count >= ability.max
            	|| ability.direction == -1 && ability.count <= 0 {
	            ability.direction = -ability.direction;
            }
            // uhhhhhhhhhhhh
            ability.count = ((ability.count as isize) + (ability.direction * 1) as isize) as usize;
            sprite.sprite_number = ability.count;
            println!("{}", ability.count);
	        let player_tf = transforms.get(ability.target).unwrap();
	        *transforms.get_mut(entity).unwrap() = player_tf.clone();
        }
    }
}


