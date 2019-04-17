extern crate amethyst;
#[macro_use]
extern crate serde;

mod network;
mod states;
mod systems;
mod components;

use network::{CustomNetEvent, NetParams};

use amethyst::{
    prelude::*,
    renderer::*,
    ecs::prelude::*,
    core::{TransformBundle, transform::Transform},
    input::InputBundle,
    assets::{AssetStorage, Loader},
    utils::{application_dir, ortho_camera::*},
    network::{NetConnection, NetworkBundle},
};

/// constants
// this is a pixelly game. the GAME resolution is gonna be 320x180 but it can be whatever size it wants
pub const GAME_WIDTH: f32 = 320.0;
pub const GAME_HEIGHT: f32 = 180.0;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(amethyst::LoggerConfig {
        stdout: amethyst::StdoutLog::Colored,
        level_filter: amethyst::LogLevelFilter::Warn,
        log_file: None,
        allow_env_override: true,
        log_gfx_device_level: None,
    });

    let path = application_dir("resources/display_config.ron")?;
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new())
    );

    let binding_path = "./resources/bindings_config.ron";
    let input_bundle = InputBundle::<String, String>::new() // TODO: change actions to a u8 and use that??
        .with_bindings_from_file(binding_path)?;

    let game_data =
        GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with(CameraOrthoSystem::default(), "letterbox", &[])
        .with(systems::input::InputSystem {}, "input", &["input_system"])
        .with(systems::ability::AbilitySystem::new(), "ability", &[])
        .with(systems::net_update::NetUpdate, "net_update", &[])
        .with(systems::net_receive::NetReceive::default(), "net_receive", &[]) // TODO: do this after NetworkBundle
        .with(systems::player_move::MoveSystem::default(), "move", &["input", "net_receive", "net_update"])
        ;

    let args: Vec<String> = std::env::args().collect();
    let is_server = if args.len() > 1 { args[1] == "server" } else { false };
    // Bind to the correct port
    let game_data = if is_server {
        game_data.with_bundle(NetworkBundle::<CustomNetEvent>::new(
            "0.0.0.0:3456".parse().unwrap(),
            vec![],
        ))?
    } else {
        game_data.with_bundle(NetworkBundle::<CustomNetEvent>::new(
            "0.0.0.0:0".parse().unwrap(),
            vec![],
        ))?
    };

    let mut game = Application::build("./", states::level_0::Level0)?
        .with_resource(NetParams {
            is_server: is_server,
            first_frame: 0,
        })
        .build(game_data)?
        ;

    game.run();

    Ok(())
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_z(1.0);
    let mut ortho = CameraOrtho::default();
    ortho.mode = CameraNormalizeMode::Contain;
    ortho.world_coordinates = CameraOrthoWorldCoordinates {
        left: 0.0,
        top: GAME_HEIGHT,
        right: GAME_WIDTH,
        bottom: 0.0,
    };
    world.create_entity()
        .with(ortho)
        .with(Camera::standard_2d())
        .with(transform)
        .build();
}

fn init_net(world: &mut World) {
    let net_params = world.read_resource::<NetParams>().clone();
    if !net_params.is_server {
        world
            .create_entity()
            .with(NetConnection::<CustomNetEvent>::new("127.0.0.1:3456".parse().unwrap()))
            .build();
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

fn load_sprite_sheet(world: &mut World, ron_path: &str, png_path: &str) -> SpriteSheetHandle {
    let texture_handle = load_texture(world, png_path);
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("resources/{}", ron_path),
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}

fn init_player(world: &mut World, is_server: bool) {
    let tex = load_texture(world, "player.png");
    let stick = load_sprite_sheet(world, "stick.ron", "stick.png");
    let stick_render = SpriteRender {
        sprite_sheet: stick,
        sprite_number: 0,
    };
    let mut transform = Transform::default();
    transform.set_translation_x(GAME_WIDTH/2.0);
    transform.set_translation_y(GAME_HEIGHT/2.0);
    let player = world.create_entity()
        // TODO: add Owner component, and use that for everything instead player property
        .with(Player::new(is_server))
        .with(transform.clone())
        .with(tex)
        .build();
    // The primary attack abilities have a usize power that goes up and down
    // q, w, e, r: attacks
    world.create_entity()
        .with(AbilityComp::new(player, AbilityType::ShortRangeDamage))
        .with(stick_render)
        .with(transform)
        .build();
}

pub struct Player {
    pub is_server: bool,
}
impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
impl Player {
    fn new(is_server: bool) -> Self {
        Player {
            is_server,
        }
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum AbilityType {
    ShortRangeDamage,
}

/// Defines a type of ability and its state
#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct AbilityState {
    ability_type: AbilityType,
    // The seconds it takes to reach max power
    pub freq: f32,
    // Current seconds count
    count: f32,
    // Currently going up or down?
    direction: i8,
}

pub struct AbilityComp {
    // The player this ability belongs to
    pub target: Entity,
    // The number of frames in the animation
    pub frames: usize,
    // The data of the ability
    state: AbilityState,
}
impl Component for AbilityComp {
    type Storage = DenseVecStorage<Self>;
}
impl AbilityComp {
    fn new(target: Entity, ability_type: AbilityType) -> Self {
	    let (freq, frames) = match ability_type {
		    AbilityType::ShortRangeDamage => (1., 4),
	    };
        Self {
            target,
            frames,
            state: AbilityState {
                ability_type,
                freq,
                count: 0.,
                direction: 1,
            }
        }
    }
}

