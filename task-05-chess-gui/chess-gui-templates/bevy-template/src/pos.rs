//! Chess GUI template.
//! Author: Lucas Jansson <lucjan@kth.se>
//! Last updated: 2026-09-24
//!
//! Position-related utility functions.

use bevy::{math::u8vec2, prelude::*};

use crate::{PIECE_SPACING, PIECE_SPRITE_SIZE};

/// Returns the [`Transform`] of a piece on the rendered board from its file and rank indices (`0..=7`).
#[must_use]
pub fn board_fr_to_transform(file: u8, rank: u8) -> Transform {
    debug_assert!(file <= 7);
    debug_assert!(rank <= 7);

    const STEP: f32 = PIECE_SPRITE_SIZE + PIECE_SPACING;

    let xy = u8vec2(file, rank)
        .as_vec2()
        .mul_add(Vec2::splat(STEP), const { Vec2::splat(STEP * -3.5) });

    Transform::from_xyz(xy.x, xy.y, 0.5)
}
