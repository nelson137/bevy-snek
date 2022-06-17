use bevy::prelude::*;

use crate::snake::SnakeDirection;

#[derive(Component)]
pub(crate) struct SnakeHead {
    pub(crate) direction: SnakeDirection,
}

#[derive(Component)]
pub(crate) struct SnakeSegment;

pub(crate) type AnySnakePiece = Or<(With<SnakeHead>, With<SnakeSegment>)>;

#[derive(Component)]
pub(crate) struct Food;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Position {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

#[derive(Component)]
pub(crate) struct ItemSize {
    pub(crate) width: f32,
    pub(crate) height: f32,
}

impl ItemSize {
    pub(crate) fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}
