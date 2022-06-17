use bevy::{audio::AudioPlugin, core::FixedTimestep, prelude::*};
use clap::Parser;

mod arena;
mod cli;
mod components;
mod events;
mod food;
mod game;
mod resources;
mod snake;

use arena::{position_translation, size_scaling};
use cli::Cli;
use events::{GameOverEvent, GrowthEvent};
use food::food_spawner;
use game::game_over;
use resources::{IsDebug, LastTailSegmentPosition, SnakeSegments};
use snake::{
    snake_eating, snake_growth, snake_movement, snake_movement_input,
    spawn_snake,
};

fn window_resize(mut windows: ResMut<Windows>) {
    windows
        .get_primary_mut()
        .unwrap()
        .set_resolution(800.0, 800.0);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn main() {
    let cli = Cli::parse();

    App::new()
        // Plugins
        .add_plugins_with(DefaultPlugins, |group| {
            group.disable::<AudioPlugin>()
        })
        // Resources
        .insert_resource(IsDebug(cli.debug))
        .insert_resource(WindowDescriptor {
            title: "Snek".into(),
            width: 500.0,
            height: 500.0,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(SnakeSegments::default())
        .insert_resource(LastTailSegmentPosition::default())
        // Events
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
        // Startup systems
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        // Systems - arena
        .add_system(window_resize)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        // Systems - snake
        .add_system(snake_movement_input.before(snake_movement))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.12))
                .with_system(snake_movement)
                .with_system(snake_eating.after(snake_movement))
                .with_system(snake_growth.after(snake_eating)),
        )
        .add_system(game_over.after(snake_movement))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.5))
                .with_system(food_spawner),
        )
        // Run
        .run();
}
