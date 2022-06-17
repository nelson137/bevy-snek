use bevy::prelude::*;
use lazy_static::lazy_static;
use rand::random;

use crate::{
    arena::{ARENA_HEIGHT, ARENA_WIDTH},
    components::{AnySnakePiece, Food, ItemSize, Position},
};

lazy_static! {
    static ref FOOD_COLOR: Color = Color::rgb_u8(199, 55, 47); // #c7372f
}

pub(crate) fn food_spawner(
    mut commands: Commands,
    items: Query<&Position, Or<(AnySnakePiece, With<Food>)>>,
) {
    let positions = items.iter().map(|p| *p).collect::<Vec<_>>();

    fn rand_int_up_to(bound: u32) -> i32 {
        (random::<f32>() * bound as f32) as i32
    }

    let mut x = rand_int_up_to(ARENA_WIDTH);
    let mut y = rand_int_up_to(ARENA_HEIGHT);

    let mut did_wrap = false;
    loop {
        // Reposition new food until an open cell is found
        while positions.contains(&Position { x, y }) {
            x += 1;
            if x >= ARENA_WIDTH as i32 {
                x = 0;
                y -= 1;
            }
        }

        // If cell is out of arena and we didn't already wrap,
        // then wrap to top and try again
        if y < 0 {
            if did_wrap {
                // No empty cells in arena, don't spawn food
                return;
            }
            y = ARENA_HEIGHT as i32 - 1;
            did_wrap = true;
            continue;
        }

        break;
    }

    // Spawn food
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: *FOOD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Food)
        .insert(Position { x, y })
        .insert(ItemSize::square(0.5));
}
