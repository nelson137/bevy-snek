use bevy::prelude::*;

use crate::{
    components::{AnySnakePiece, Food},
    events::GameOverEvent,
    resources::SnakeSegments,
    snake::spawn_snake,
};

pub(crate) fn game_over(
    mut commands: Commands,
    mut game_over_reader: EventReader<GameOverEvent>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, AnySnakePiece>,
) {
    if game_over_reader.iter().next().is_some() {
        println!("GAME OVER!");
        // Despawn all entities
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        // Restart game, start new snake
        spawn_snake(commands, segments_res);
    }
}
