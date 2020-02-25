use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::pong::{ARENA_HEIGHT, Paddle, PADDLE_HEIGHT, Side};

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'a> System<'a> for PaddleSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Paddle>,
        Read<'a, InputHandler<StringBindings>>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut transforms, paddles, input, time): Self::SystemData) {
        for (transform, paddle) in (&mut transforms, &paddles).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };
            if let Some(move_amount) = movement {
                if move_amount != 0.0 {
                    let scaled_amount = move_amount * 100.0 * time.delta_seconds();
                    transform.set_translation_y(
                        (transform.translation().y + scaled_amount)
                            .max(0.5 * PADDLE_HEIGHT)
                            .min(ARENA_HEIGHT - 0.5 * PADDLE_HEIGHT),
                    );
                }
            }
        }
    }
}
