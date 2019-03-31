use amethyst::{
    prelude::*,
};

use {load_texture, init_camera, init_image, init_net, init_player};

pub struct Level0;

impl SimpleState for Level0 {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        let texture_handle = load_texture(world, "bg.png");
        init_image(world, &texture_handle);
        init_camera(world);
        init_net(world);
        // MAYBE: use IDs? MAYBE: use a different enum for characters?
        init_player(world, true);
        init_player(world, false);
    }
}

