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

use {Player, UpdateEvent};

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
        WriteStorage<'a, NetConnection<UpdateEvent>>,
    );
    fn run(&mut self, (mut connections): Self::SystemData) {
        // TODO :Delete
        for (mut conn,) in (&mut connections,).join() {
            if !self.readers.contains_key(&conn.target) {
                self.readers.insert(conn.target, conn.receive_buffer.register_reader());
            }
            // TODO: This is a hack, listen to NetEvent::Connect!!
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

