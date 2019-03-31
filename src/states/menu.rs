use amethyst::{
    prelude::*,
    core::{Transform},
    ecs::{Entity},
    renderer::{TextureHandle, Texture},
};

use {load_texture, init_camera, GAME_WIDTH, GAME_HEIGHT};

pub struct Menu;

impl SimpleState for Menu {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        let texture_handle = load_texture(world, "bg.png");
        init_image(world, &texture_handle);
        init_camera(world);
    }
}

fn init_image(world: &mut World, texture: &TextureHandle) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_x(GAME_WIDTH/2.0);
    transform.set_translation_y(GAME_HEIGHT/2.0);
    world.create_entity()
        .with(transform)
        .with(texture.clone())
        .build()
}

