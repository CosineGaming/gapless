use amethyst::{
    prelude::*,
    core::{Transform},
    ecs::{Entity},
    renderer::{TextureHandle, Texture},
};

use {load_texture, init_camera, GAME_WIDTH, GAME_HEIGHT, init_image, init_player, init_net};

pub struct Level0;

impl<'a, 'b> SimpleState<'a, 'b> for Level0 {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        let texture_handle = load_texture(world, "bg.png");
        init_image(world, &texture_handle);
        init_camera(world);
        let texture_handle = load_texture(world, "player.png");
        init_player(world, &texture_handle);
        init_net(world);
    }
}

