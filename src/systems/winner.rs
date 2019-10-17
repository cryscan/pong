use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, System, SystemData, World, WriteStorage},
};

use crate::pong::{ARENA_WIDTH, Ball};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'a> System<'a> for WinnerSystem {
    type SystemData = (
        WriteStorage<'a, Ball>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (mut balls, mut locals): Self::SystemData) {
        for (ball, local) in (&mut balls, &mut locals).join() {
            let ball_x = local.translation().x;

            let did_hit = if ball_x <= -ball.radius {
                println!("Player 2 Scores!");
                true
            } else if ball_x >= ARENA_WIDTH + ball.radius {
                println!("Player 1 Scores!");
                true
            } else { false };

            if did_hit {
                ball.velocity[0] = -ball.velocity[0];
                local.set_translation_x(ARENA_WIDTH * 0.5);
            }
        }
    }
}