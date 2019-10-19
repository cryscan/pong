use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, ReadExpect, System, SystemData, World, Write, WriteStorage},
    ui::UiText,
};

use crate::pong::{ARENA_WIDTH, Ball, ScoreBoard, ScoreText};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'a> System<'a> for WinnerSystem {
    type SystemData = (
        WriteStorage<'a, Ball>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, UiText>,
        Write<'a, ScoreBoard>,
        ReadExpect<'a, ScoreText>,
    );

    fn run(&mut self, (
        mut balls,
        mut locals,
        mut ui_text,
        mut scores,
        score_text,
    ): Self::SystemData) {
        for (ball, local) in (&mut balls, &mut locals).join() {
            let ball_x = local.translation().x;

            let did_hit = if ball_x <= -ball.radius {
                scores.score_right = (scores.score_right + 1).min(999);
                if let Some(text) = ui_text.get_mut(score_text.p2_score) {
                    text.text = scores.score_right.to_string();
                }
                true
            } else if ball_x >= ARENA_WIDTH + ball.radius {
                scores.score_left = (scores.score_left + 1).min(999);
                if let Some(text) = ui_text.get_mut(score_text.p1_score) {
                    text.text = scores.score_left.to_string();
                }
                true
            } else { false };

            if did_hit {
                ball.velocity[0] = -ball.velocity[0];
                local.set_translation_x(ARENA_WIDTH * 0.5);
            }
        }
    }
}