use amethyst::core::{Transform, Time};
use amethyst::core::nalgebra::Vector3;
use amethyst::ecs::{Join, Read, Write, ReadExpect, ReadStorage, System, WriteStorage};
use amethyst::network::{NetConnection, NetEvent};
use amethyst::input::InputHandler;
use amethyst::shrev::EventChannel;

use {UpdateEvent, ClientEvent, Player, NetParams, GAME_WIDTH, GAME_HEIGHT};

pub struct ClientUpdate;

impl<'s> System<'s> for ClientUpdate {
    type SystemData = (
        WriteStorage<'s, NetConnection<UpdateEvent>>,
        Write<'s, EventChannel<ClientEvent>>,
        Read<'s, Time>,
        Read<'s, InputHandler<String, String>>,
        ReadExpect<'s, NetParams>,
    );

    fn run(&mut self, (mut connections, mut channel, time, input, net_params): Self::SystemData) {
        let client_event = ClientEvent {
            frame: time.frame_number(),
            id: net_params.id,
            up: input.action_is_down("up").unwrap(),
            down: input.action_is_down("down").unwrap(),
            left: input.action_is_down("left").unwrap(),
            right: input.action_is_down("right").unwrap(),
        };
        let update_event = NetEvent::Custom(UpdateEvent::Client(client_event.clone()));
        for mut conn in (&mut connections).join() {
            conn.send_buffer.single_write(update_event.clone());
        }
        channel.single_write(client_event);
    }
}

