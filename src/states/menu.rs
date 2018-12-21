use amethyst::{
    prelude::*,
    core::{Transform},
    ecs::{Entity},
    renderer::{TextureHandle, Texture},
    ui::UiButtonBuilder,
};

use {load_texture, init_image, init_camera, GAME_WIDTH, GAME_HEIGHT};

pub struct Menu;

impl<'a, 'b> SimpleState<'a, 'b> for Menu {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        let texture_handle = load_texture(world, "menu.png");
        init_image(world, &texture_handle);
        init_camera(world);

        let play = UiButtonBuilder::new("doot", "textttt")
            .with_position(160., -100.)
            .build_from_world(&world);
    }
}

