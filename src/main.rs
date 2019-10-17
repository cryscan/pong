use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        RenderingBundle,
        types::DefaultBackend,
    },
    utils::application_root_dir,
};

use crate::pong::Pong;

mod pong;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let config_dir = app_root.join("config");
    let assets_dir = app_root.join("assets");

    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(RenderToWindow::from_config_path(config_dir.join("display.ron"))
            .with_clear([0.0, 0.0, 0.0, 1.0]))
        .with_plugin(RenderFlat2D::default());

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(config_dir.join("bindings.ron"))?;

    let game_data = GameDataBuilder::default()
        .with_bundle(rendering_bundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallSystem, "ball_system", &[])
        .with(systems::BounceSystem, "collision_system",
              &["paddle_system", "ball_system"]);

    let mut game = Application::new(assets_dir, Pong, game_data)?;
    game.run();

    Ok(())
}