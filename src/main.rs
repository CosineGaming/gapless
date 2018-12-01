extern crate amethyst;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use amethyst::{
    prelude::*,
    renderer::*,
    ecs::prelude::*,
    core::{TransformBundle, transform::Transform},
    core::nalgebra::{Vector3, Vector2},
    assets::{AssetStorage, Loader},
    utils::{application_root_dir, ortho_camera::*},
    network::{NetConnection, NetworkBundle, NetEvent},
};

mod states;
mod systems;

/// constants
// this is a pixelly game. the GAME resolution is gonna be 320x180 but it can be whatever size it wants
pub const GAME_WIDTH: f32 = 320.0;
pub const GAME_HEIGHT: f32 = 180.0;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!(
        "{}/resources/display_config.ron",
        application_root_dir()
    );
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawFlat2D::new())
    );

    let game_data =
        GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with(CameraOrthoSystem::default(), "letterbox", &[])
        .with(systems::player::PlayerSystem, "player", &["input_system"])
        ;

    let args: Vec<String> = std::env::args().collect();
    let is_server = args[1] == "server";
    // Bind to the correct port
    let game_data = if is_server {
        game_data.with_bundle(NetworkBundle::<UpdateEvent>::new(
                "127.0.0.1:3456".parse().unwrap(),
                vec![],
            ))?
    } else {
        game_data.with_bundle(NetworkBundle::<UpdateEvent>::new(
                "127.0.0.1:3455".parse().unwrap(),
                vec![],
            ))?
    };

    let mut game = Application::build("./", states::menu::Menu)?
        .with_resource(NetParams {
            is_server: is_server,
            id: 0, // TODO: Use IP or somethivgn
        })
        .build(game_data)?
        ;

    game.run();

    Ok(())
}

fn load_texture(world: &mut World, png_path: &str) -> TextureHandle {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
        format!("texture/{}", png_path),
        PngFormat,
        TextureMetadata::srgb_scale(),
        (),
        &texture_storage,
    )
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world.create_entity()
        .with(CameraOrtho {
            mode: CameraNormalizeMode::Contain,
            world_coordinates: CameraOrthoWorldCoordinates {
                left: 0.0,
                top: GAME_HEIGHT,
                right: GAME_WIDTH,
                bottom: 0.0,
            }
        })
        .with(Camera::standard_2d())
        .with(transform)
        .build();
}

fn init_net(world: &mut World) {
    let address = {
        let net_params = world.read_resource::<NetParams>();
        if net_params.is_server {
            "127.0.0.1:3455"
        } else {
            "127.0.0.1:3456"
        }.parse().unwrap()
    };
    world
        .create_entity()
        .with(NetConnection::<UpdateEvent>::new(address))
        .build();
}

fn init_image(world: &mut World, texture: &TextureHandle) -> Entity {
    let mut transform = Transform::default();
    transform.set_x(GAME_WIDTH/2.0);
    transform.set_y(GAME_HEIGHT/2.0);
    world.create_entity()
        .with(transform)
        .with(texture.clone())
        .build()
}

fn init_player(world: &mut World, texture: &TextureHandle) -> Entity {
    let mut transform = Transform::default();
    transform.set_x(GAME_WIDTH/2.0);
    transform.set_y(GAME_HEIGHT/2.0);
    world.create_entity()
        .with(transform)
        .with(texture.clone())
        .build()
}

pub struct Player {
    pub id: u32,
}

impl Player {
    fn new(id: u32) -> Self {
        Player {
            id: id
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

// This should probly be in different file but
pub struct NetParams {
    pub is_server: bool,
    // TODO: How do we turn IP to a u32?
    // a net id
    pub id: u32,
}

// Sent every frame by the server to update on the state of the world
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateEvent {
    pub frame: u64,
    pub tfs: Vec<TFEvent>,
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct TFEvent {
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
}

