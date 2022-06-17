use bevy::prelude::*;

use crate::components::{ItemSize, Position};

pub(crate) const ARENA_WIDTH: u32 = 20;
pub(crate) const ARENA_HEIGHT: u32 = 20;

pub(crate) fn size_scaling(
    windows: Res<Windows>,
    mut q: Query<(&ItemSize, &mut Transform)>,
) {
    let win = windows.get_primary().unwrap(); // FIXME
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width * win.width() / ARENA_WIDTH as f32,
            sprite_size.height * win.height() / ARENA_HEIGHT as f32,
            1.0,
        );
    }
}

pub(crate) fn position_translation(
    windows: Res<Windows>,
    mut q: Query<(&Position, &mut Transform)>,
) {
    /// Convert an arena grid-wise coordinate into a window coordinate.
    ///
    /// - `pos`: arena grid coord
    /// - `bound_win`: window bounds (width/height) in pixels
    /// - `bound_game`: grid bounds (width/height)
    fn convert(pos: f32, bound_win: f32, bound_game: f32) -> f32 {
        let tile_size = bound_win / bound_game;
        pos * tile_size - (bound_win / 2.0) + (tile_size / 2.0)
    }

    let win = windows.get_primary().unwrap(); // FIXME
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, win.width(), ARENA_WIDTH as f32),
            convert(pos.y as f32, win.height(), ARENA_HEIGHT as f32),
            0.0,
        );
    }
}
