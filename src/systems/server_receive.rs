use amethyst::{
    ecs::prelude::*,
    network::*,
    assets::{AssetStorage, Loader},
    renderer::{TextureMetadata, Texture, TextureHandle, PngFormat},
    core::nalgebra::{Vector3},
    core::Transform,
    shrev::EventChannel,
};

use std::collections::HashMap;

use {Player, UpdateEvent, ClientEvent};

/// A simple system that receives a ton of network events.
pub struct ServerReceive {
    pub readers: HashMap<std::net::SocketAddr, ReaderId<NetEvent<UpdateEvent>>>,
}

impl ServerReceive {
    pub fn new() -> Self {
        ServerReceive { readers: HashMap::new() }
    }
}

impl<'a> System<'a> for ServerReceive {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, NetConnection<UpdateEvent>>,
        Write<'a, EventChannel<ClientEvent>>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Transform>,
        ReadExpect<'a, AssetStorage<Texture>>,
        WriteStorage<'a, TextureHandle>,
        ReadExpect<'a, Loader>,
    );
    fn run(&mut self, (entities, mut connections, mut channel, mut players, mut transforms, texture_storage, mut texture_handles, loader): Self::SystemData) {
        for (mut conn,) in (&mut connections,).join() {
            let mut new_connection = false;
            if !self.readers.contains_key(&conn.target) {
                self.readers.insert(conn.target, conn.receive_buffer.register_reader());
                new_connection = true;
            }
            println!("{}", conn.target);
            // TODO: This is a hack, listen to NetEvent::Connect!!
            if new_connection {
                // Spawn a player etc
                // TODO: Fix code duplication with init_player and state.on_start
                let texture_handle = load_texture("player.png", &loader, &*texture_storage);
                let mut transform = Transform::default();
                transform.set_x(100.0); // TODO: Where???
                transform.set_y(40.0);
                entities.build_entity()
                    .with(Player::new(0), &mut players) // TODO: id
                    .with(transform, &mut transforms)
                    .with(texture_handle.clone(), &mut texture_handles)
                    .build();
            }
            for ev in conn.receive_buffer.read(self.readers.get_mut(&conn.target).unwrap()) {
                match ev {
                    NetEvent::Custom(event) => {
                        if let UpdateEvent::Client(client_event) = event {
                            channel.single_write(client_event.clone());
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}

fn load_texture(png_path: &str, loader: &Loader, texture_storage: &AssetStorage<Texture>) -> TextureHandle {
    loader.load(
        format!("texture/{}", png_path),
        PngFormat,
        TextureMetadata::srgb_scale(),
        (),
        &texture_storage,
    )
}

