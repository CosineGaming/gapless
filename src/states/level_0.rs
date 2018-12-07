use amethyst::{
    prelude::*,
    core::{Transform},
    ecs::{Entity},
    renderer::{TextureHandle, Texture},
};

use {load_texture, init_camera, GAME_WIDTH, GAME_HEIGHT, init_image, init_net, init_player};

pub struct Level0;

impl<'a, 'b> SimpleState<'a, 'b> for Level0 {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        let texture_handle = load_texture(world, "bg.png");
        init_image(world, &texture_handle);
        init_camera(world);
        init_net(world);
        let player_tex = load_texture(world, "player.png");
        // MAYBE: use IDs? MAYBE: use a different enum for characters?
        init_player(world, &player_tex.clone(), true);
        init_player(world, &player_tex, false);
    }
}

