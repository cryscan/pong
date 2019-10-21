use std::ops::Deref;

use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::audio::{play_bounce_sound, Sounds};
use crate::pong::{ARENA_HEIGHT, Ball, Paddle, Side};

#[derive(SystemDesc)]
pub struct BounceSystem;

impl<'a> System<'a> for BounceSystem {
    type SystemData = (
        WriteStorage<'a, Ball>,
        ReadStorage<'a, Paddle>,
        ReadStorage<'a, Transform>,
        Read<'a, AssetStorage<Source>>,
        ReadExpect<'a, Sounds>,
        Option<Read<'a, Output>>,
    );

    fn run(&mut self, (
        mut balls,
        paddles,
        transforms,
        storage,
        sounds,
        audio_output,
    ): Self::SystemData) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            if (ball_y <= ball.radius && ball.velocity[1] < 0.)
                || (ball_y >= ARENA_HEIGHT - ball.radius && ball.velocity[1] > 0.) {
                ball.velocity[1] = -ball.velocity[1];
                play_bounce_sound(
                    &*sounds,
                    &storage,
                    audio_output.as_ref().map(|o| o.deref()),
                );
            }

            for (paddle, transform) in (&paddles, &transforms).join() {
                let paddle_x = transform.translation().x - paddle.width * 0.5;
                let paddle_y = transform.translation().y - paddle.height * 0.5;

                if point_in_rect(
                    ball_x,
                    ball_y,
                    paddle_x - ball.radius,
                    paddle_y - ball.radius,
                    paddle_x + paddle.width + ball.radius,
                    paddle_y + paddle.height + ball.radius,
                ) && paddle_ball_collide(paddle, ball) {
                    ball.velocity[0] = -ball.velocity[0];
                    play_bounce_sound(
                        &*sounds,
                        &storage,
                        audio_output.as_ref().map(|o| o.deref()),
                    );
                }
            }
        }
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y <= top && y >= bottom
}

fn paddle_ball_collide(paddle: &Paddle, ball: &Ball) -> bool {
    (paddle.side == Side::Left && ball.velocity[0] < 0.)
        || (paddle.side == Side::Right && ball.velocity[0] > 0.)
}