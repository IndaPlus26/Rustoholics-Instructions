//! Chess GUI template.
//! Author: Lucas Jansson <lucjan@kth.se>
//! Last updated: 2026-09-24
//!
//! Observer systems.

use bevy::prelude::*;

use crate::Piece;

/// Does something when a piece is clicked.
pub fn on_click_piece(event: On<Pointer<Click>>, pieces: Query<&Piece>) {
    if let Ok(piece) = pieces.get(event.event().event_target()) {
        info!(
            "A piece ({piece:?}) has been clicked at {:?}!",
            event.pointer_location
        );
    }
}
