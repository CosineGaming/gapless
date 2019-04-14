use amethyst::{
    ecs::prelude::*,
    core::Time,
    input::InputHandler,
    network::{NetEvent, NetConnection},
};

use crate::network::{CustomNetEvent, AnyEvent, NetParams, InputEvent};

pub struct InputSystem;

impl<'s> System<'s> for InputSystem {
    type SystemData = (
        Read<'s, InputHandler<String, String>>,
        WriteStorage<'s, NetConnection<CustomNetEvent>>,
        ReadExpect<'s, NetParams>,
        Read<'s, Time>,
    );

    fn run(&mut self, (input, mut conns, net_params, time): Self::SystemData) {
        let input_event = InputEvent {
            left: input.action_is_down("left").unwrap(),
            right: input.action_is_down("right").unwrap(),
            up: input.action_is_down("up").unwrap(),
            down: input.action_is_down("down").unwrap(),
        };
        let net_event = NetEvent::Reliable(CustomNetEvent {
            event: AnyEvent::Input(input_event),
            frame: time.frame_number(),
            from_server: net_params.is_server,
        });
        for conn in (&mut conns).join() {
            conn.send_buffer.single_write(net_event.clone());
        }
    }
}

