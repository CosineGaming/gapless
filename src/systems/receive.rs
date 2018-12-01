use amethyst::{
    ecs::{Join, ReaderId, System, ReadStorage, WriteStorage},
    network::*,
    core::cgmath::{Vector3},
    core::Transform,
};

use game::{Paddle, Ball, UpdateEvent, Side, ARENA_WIDTH};

/// A simple system that receives a ton of network events.
pub struct ReceiveSystem {
    pub reader: Option<ReaderId<NetEvent<UpdateEvent>>>,
}

impl ReceiveSystem {
    pub fn new() -> Self {
        ReceiveSystem { reader: None }
    }
}

impl<'a> System<'a> for ReceiveSystem {
    type SystemData = (
        WriteStorage<'a, NetConnection<UpdateEvent>>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Paddle>,
        WriteStorage<'a, Ball>,
    );
    fn run(&mut self, (mut connections, mut transforms, paddles, mut balls): Self::SystemData) {
        for (mut conn,) in (&mut connections,).join() {
            if self.reader.is_none() {
                self.reader = Some(conn.receive_buffer.register_reader());
            }
            let mut iter = conn.receive_buffer.read(self.reader.as_mut().unwrap())
                .filter_map(|ev| {
                    if let NetEvent::Custom(event) = ev {
                        Some(event)
                    } else {
                        None
                    }
                });
            if let Some(first) = iter.next() {
                let recent = iter.fold(first, |acc, ev| {
                    if acc.frame > ev.frame { acc } else { ev }
                });
                for (paddle, mut transform) in (&paddles, &mut transforms).join() {
                    if paddle.side == Side::Right {
                        transform.translation[1] = recent.paddle.vertical;
                    }
                }
                for (mut ball, mut transform) in (&mut balls, &mut transforms).join() {
                    for ball_event in &recent.balls {
                        // The stage is flipped from the other
                        ball.velocity = [-ball_event.velocity[0], ball_event.velocity[1]];
                        transform.translation = Vector3::new(ARENA_WIDTH - ball_event.position.x, ball_event.position.y, 0.0);
                    }
                }
            }
        }
    }
}

