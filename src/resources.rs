use std::ops::Deref;

use bevy::prelude::*;

use crate::components::Position;

pub(crate) struct IsDebug(pub(crate) bool);

impl Deref for IsDebug {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Default)]
pub(crate) struct DebugSnakePosition(pub(crate) Position);

#[derive(Default)]
pub(crate) struct SnakeSegments(pub(crate) Vec<Entity>);

#[derive(Default)]
pub(crate) struct LastTailSegmentPosition(pub(crate) Option<Position>);
