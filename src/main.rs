#[macro_use]
extern crate log;
#[macro_use]
extern crate specs_derive;

use amethyst::{
    animation::AnimationBundle,
    assets::{PrefabLoaderSystem, Processor},
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    renderer::{
        sprite::{SpriteRender, SpriteSheet},
        types::DefaultBackend,
        RenderingSystem,
    },
    utils::application_root_dir,
    window::WindowBundle,
    Application, GameDataBuilder,
};

mod components;
mod entities;
mod graph_creator;
mod resources;
mod states;
mod systems;

use components::{AnimationId, AnimationPrefabData};
use resources::Map;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let root = application_root_dir()?;
    let display_config_path = root.join("resources/display_config.ron");
    let assets_path = root.join("assets");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(root.join("resources/bindings_config.ron"))?;

    let game_data = GameDataBuilder::default()
        .with_bundle(WindowBundle::from_config_path(display_config_path))?
        .with(
            PrefabLoaderSystem::<AnimationPrefabData>::default(),
            "scene_loader",
            &[],
        )
        .with_bundle(AnimationBundle::<AnimationId, SpriteRender>::new(
            "sprite_animation_control",
            "sprite_sampler_interpolation",
        ))?
        .with_bundle(
            TransformBundle::new()
                .with_dep(&["sprite_animation_control", "sprite_sampler_interpolation"]),
        )?
        .with_bundle(input_bundle)?
        .with(
            Processor::<SpriteSheet>::new(),
            "sprite_sheet_processor",
            &[],
        )
        .with(Processor::<Map>::new(), "map_processor", &[])
        .with_thread_local(RenderingSystem::<DefaultBackend, _>::new(
            graph_creator::GameGraphCreator::default(),
        ));
    let mut game =
        Application::build(assets_path, states::LoadState::default())?.build(game_data)?;

    game.run();

    Ok(())
}
