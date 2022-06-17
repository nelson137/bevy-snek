use std::sync::atomic::{AtomicUsize, Ordering};

use bevy::prelude::*;
use lazy_static::lazy_static;

use crate::{
    arena::{ARENA_HEIGHT, ARENA_WIDTH},
    components::{Food, ItemSize, Position, SnakeHead, SnakeSegment},
    events::{GameOverEvent, GameOverReason, GrowthEvent},
    resources::{
        DebugSnakePosition, IsDebug, LastTailSegmentPosition, SnakeSegments,
    },
};

lazy_static! {
    static ref SNAKE_HEAD_COLOR: Color = Color::rgb_u8(162, 214, 173); // #a2d6ad
    static ref SNAKE_SEGMENT_COLORS: [Color; 6] = [
        // manual
        // Color::rgb_u8(158, 207, 52), // #9ecf34
        // Color::rgb_u8(127, 179, 53), // #7fb335
        // Color::rgb_u8(63, 143, 41), // #3f8f29
        // Color::rgb_u8(85, 120, 34), // #557822

        // darken 5% each step
        // Color::rgb_u8(63, 143, 41), // #3f8f29
        // Color::rgb_u8(59, 135, 38), // #3b8726
        // Color::rgb_u8(56, 128, 36), // #388024
        // Color::rgb_u8(53, 121, 34), // #357922
        // Color::rgb_u8(50, 114, 32), // #327220

        // darken 10% each step
        Color::rgb_u8(74, 169, 48), // #4aa830
        Color::rgb_u8(63, 143, 41), // #3f8f29
        Color::rgb_u8(56, 128, 36), // #388024
        Color::rgb_u8(50, 114, 32), // #327220
        Color::rgb_u8(44, 100, 28), // #2c641c
        Color::rgb_u8(37, 85, 24), // #255518
    ];
}

static SNAKE_SEGMENT_COLOR_I: AtomicUsize = AtomicUsize::new(0);

pub(crate) fn get_snake_segment_color() -> Color {
    let i = SNAKE_SEGMENT_COLOR_I.fetch_add(1, Ordering::Relaxed);
    SNAKE_SEGMENT_COLORS[i % SNAKE_SEGMENT_COLORS.len()]
}

#[derive(Copy, Clone, PartialEq)]
pub(crate) enum SnakeDirection {
    Up,
    Right,
    Down,
    Left,
}

impl SnakeDirection {
    fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }
}

pub(crate) fn spawn_snake(
    mut commands: Commands,
    mut segments: ResMut<SnakeSegments>,
) {
    let y = ARENA_WIDTH as i32 / 2;
    *segments = SnakeSegments(vec![
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: *SNAKE_HEAD_COLOR,
                    ..default()
                },
                ..default()
            })
            .insert(SnakeHead {
                direction: SnakeDirection::Right,
            })
            .insert(Position { x: 3, y })
            .insert(ItemSize::square(0.95))
            .id(),
        spawn_segment(commands, Position { x: 2, y }),
    ]);
}

pub(crate) fn snake_movement_input(
    keyboard: Res<Input<KeyCode>>,
    mut heads: Query<&mut SnakeHead>,
) {
    if let Some(mut head) = heads.iter_mut().next() {
        let next_dir = if keyboard.any_pressed([KeyCode::Up, KeyCode::W]) {
            SnakeDirection::Up
        } else if keyboard.any_pressed([KeyCode::Right, KeyCode::D]) {
            SnakeDirection::Right
        } else if keyboard.any_pressed([KeyCode::Down, KeyCode::S]) {
            SnakeDirection::Down
        } else if keyboard.any_pressed([KeyCode::Left, KeyCode::A]) {
            SnakeDirection::Left
        } else {
            head.direction
        };
        // Prevent flipping back onto body
        if next_dir != head.direction.opposite() {
            head.direction = next_dir;
        }
    }
}

pub(crate) fn snake_movement(
    is_debug: Res<IsDebug>,
    mut debug_snake_pos: ResMut<DebugSnakePosition>,
    segments: ResMut<SnakeSegments>,
    mut game_over_writer: EventWriter<GameOverEvent>,
    mut last_tail_pos: ResMut<LastTailSegmentPosition>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        // Store positions of full snake
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();

        *last_tail_pos =
            LastTailSegmentPosition(Some(*segment_positions.last().unwrap()));

        // Update head position
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match head.direction {
            SnakeDirection::Up => {
                head_pos.y += 1;
            }
            SnakeDirection::Right => {
                head_pos.x += 1;
            }
            SnakeDirection::Down => {
                head_pos.y -= 1;
            }
            SnakeDirection::Left => {
                head_pos.x -= 1;
            }
        }

        if **is_debug {
            *debug_snake_pos = DebugSnakePosition(*head_pos);
        } else {
            if head_pos.x < 0
                || head_pos.x >= ARENA_WIDTH as i32
                || head_pos.y < 0
                || head_pos.y >= ARENA_HEIGHT as i32
            {
                game_over_writer
                    .send(GameOverEvent(GameOverReason::SnakeHitWall));
            } else if segment_positions.contains(&head_pos) {
                game_over_writer
                    .send(GameOverEvent(GameOverReason::SnakeHitSelf));
            }
        }

        // Set the pos of each segment (except head) to that of the segment
        // infront of it
        segment_positions
            .iter()
            .zip(segments.0.iter().skip(1))
            .for_each(|(pos, tail_seg)| {
                *positions.get_mut(*tail_seg).unwrap() = *pos
            });
    }
}

fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: get_snake_segment_color(),
                ..default()
            },
            ..default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(ItemSize::square(0.85))
        .id()
}

pub(crate) fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    if let Some(head_pos) = head_positions.iter().next() {
        for (ent, food_pos) in food_positions.iter() {
            if head_pos == food_pos {
                commands.entity(ent).despawn();
                growth_writer.send_default();
            }
        }
    }
}

pub(crate) fn snake_growth(
    commands: Commands,
    last_tail_pos: ResMut<LastTailSegmentPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if growth_reader.iter().next().is_some() {
        segments
            .0
            .push(spawn_segment(commands, last_tail_pos.0.unwrap()));
    }
}
