//! Chess GUI template.
//! Author: Lucas Jansson <lucjan@kth.se>
//! Last updated: 2026-09-24

use bevy::{prelude::*, window::WindowResolution};

mod observer;
mod pos;
mod scheduled;

/// Window background color.
const BACKGROUND: Color = Color::linear_rgb(0.004, 0.006, 0.008);

/// Size in texels of each piece's sprite.
const PIECE_SPRITE_SIZE: f32 = 45.0;

/// Spacing in texels between pieces' sprites.
const PIECE_SPACING: f32 = PIECE_SPRITE_SIZE / 8.0;

/// World space size of the board's sprite.
const BOARD_SIZE: f32 = (PIECE_SPRITE_SIZE + PIECE_SPACING) * 8.0;

/// World space margin around the board.
const MARGIN: f32 = PIECE_SPRITE_SIZE / 2.0;

/// Data of the current game.
#[derive(Resource)]
struct Game(/* TODO */);

impl Default for Game {
    fn default() -> Self {
        Game(/* TODO: Initial game data. */)
    }
}

/// Marker component for the board.
#[derive(Clone, Component, Copy, Default, Eq, Hash, PartialEq)]
#[component(immutable)]
struct Board;

/// Component attached to all pieces.
#[derive(Clone, Component, Copy, Debug, Eq, Hash, PartialEq)]
#[component(immutable)]
struct Piece(Colour);

/// The color of a piece.
///
/// Maybe you want to replace this with an enum from the chess library.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Colour {
    Black,
    White,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Schack".to_string(),
                        resolution: WindowResolution::new(720, 720),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()), // Make sure that the pieces are rendered without interpolation.
        )
        .add_systems(Startup, scheduled::sys_startup)
        .add_observer(observer::on_click_piece)
        .init_resource::<Game>()
        .insert_resource(ClearColor(BACKGROUND))
        .run();
}
