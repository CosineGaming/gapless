use amethyst::{
    ecs::{Join, Read, ReadExpect, System, ReadStorage, WriteStorage},
    network::*,
    core::*,
};

use game::{Paddle, Ball, Side, UpdateEvent, PaddleEvent, BallEvent, NetParams};

/// A simple system that sends a ton of messages to all connections.
/// In this case, only the server is connected.
pub struct SendSystem {}

impl SendSystem {
    pub fn new() -> Self {
        SendSystem {}
    }
}

impl<'a> System<'a> for SendSystem {
    type SystemData = (
        WriteStorage<'a, NetConnection<UpdateEvent>>,
        Read<'a, Time>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Paddle>,
        ReadStorage<'a, Ball>,
        ReadExpect<'a, NetParams>,
    );
    fn run(&mut self, (mut connections, time, transforms, paddles, balls, net_params): Self::SystemData) {
        for mut conn in (&mut connections).join() {
            let opt_paddle = (&paddles, &transforms).join().fold(None, |acc, (paddle, transform)| {
                // Only send my side which is always left
                if paddle.side == Side::Left {
                    Some(PaddleEvent {
                        vertical: transform.translation[1],
                    })
                } else { acc }
            });
            let paddle = opt_paddle.unwrap();
            let mut ball_events = vec![];
            if net_params.is_server {
                for (ball, transform) in (&balls, &transforms).join() {
                    ball_events.push(BallEvent {
                        position: transform.translation.truncate(),
                        velocity: ball.velocity,
                    });
                }
            }
            let update_event = NetEvent::Custom(UpdateEvent {
                frame: time.frame_number(),
                paddle: paddle,
                balls: ball_events,
            });
            conn.send_buffer.single_write(update_event);
        }
    }
}

