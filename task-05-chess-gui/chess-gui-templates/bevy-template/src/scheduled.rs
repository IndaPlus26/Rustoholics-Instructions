//! Chess GUI template.
//! Author: Lucas Jansson <lucjan@kth.se>
//! Last updated: 2026-09-24
//!
//! Scheduled systems.

use std::path::Path;

use bevy::{camera, prelude::*};

use crate::{BOARD_SIZE, Board, Colour, MARGIN, Piece, pos::board_fr_to_transform};

/// System that runs once during startup of the game.
pub fn sys_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the camera.
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: camera::ScalingMode::AutoMin {
                min_width: const { BOARD_SIZE + MARGIN },
                min_height: const { BOARD_SIZE + MARGIN },
            },
            near: -1.0,
            far: 1.0,
            ..OrthographicProjection::default_2d()
        }),
    ));

    // Spawn the board.
    let mut board = commands.spawn((
        Sprite {
            image: asset_server.load(Path::new("board.png")),
            custom_size: Some(Vec2::splat(BOARD_SIZE)),
            ..default()
        },
        Board,
    ));

    let black_bishop = Sprite::from_image(asset_server.load(Path::new("piece/black_bishop.png")));
    let black_king = Sprite::from_image(asset_server.load(Path::new("piece/black_king.png")));
    let black_knight = Sprite::from_image(asset_server.load(Path::new("piece/black_knight.png")));
    let black_pawn = Sprite::from_image(asset_server.load(Path::new("piece/black_pawn.png")));
    let black_queen = Sprite::from_image(asset_server.load(Path::new("piece/black_queen.png")));
    let black_rook = Sprite::from_image(asset_server.load(Path::new("piece/black_rook.png")));

    let white_bishop = Sprite::from_image(asset_server.load(Path::new("piece/white_bishop.png")));
    let white_king = Sprite::from_image(asset_server.load(Path::new("piece/white_king.png")));
    let white_knight = Sprite::from_image(asset_server.load(Path::new("piece/white_knight.png")));
    let white_pawn = Sprite::from_image(asset_server.load(Path::new("piece/white_pawn.png")));
    let white_queen = Sprite::from_image(asset_server.load(Path::new("piece/white_queen.png")));
    let white_rook = Sprite::from_image(asset_server.load(Path::new("piece/white_rook.png")));

    let white = Piece(Colour::White);
    let black = Piece(Colour::Black);

    // Spawn all the pieces as children of the board.
    board.with_children(|child_spawner_commands| {
        [
            // Rank 1
            (board_fr_to_transform(0, 0), white, white_rook.clone()),
            (board_fr_to_transform(1, 0), white, white_knight.clone()),
            (board_fr_to_transform(2, 0), white, white_bishop.clone()),
            (board_fr_to_transform(3, 0), white, white_queen),
            (board_fr_to_transform(4, 0), white, white_king),
            (board_fr_to_transform(5, 0), white, white_bishop),
            (board_fr_to_transform(6, 0), white, white_knight),
            (board_fr_to_transform(7, 0), white, white_rook),
            // Rank 2
            (board_fr_to_transform(0, 1), white, white_pawn.clone()),
            (board_fr_to_transform(1, 1), white, white_pawn.clone()),
            (board_fr_to_transform(2, 1), white, white_pawn.clone()),
            (board_fr_to_transform(3, 1), white, white_pawn.clone()),
            (board_fr_to_transform(4, 1), white, white_pawn.clone()),
            (board_fr_to_transform(5, 1), white, white_pawn.clone()),
            (board_fr_to_transform(6, 1), white, white_pawn.clone()),
            (board_fr_to_transform(7, 1), white, white_pawn),
            // Rank 7
            (board_fr_to_transform(0, 6), black, black_pawn.clone()),
            (board_fr_to_transform(1, 6), black, black_pawn.clone()),
            (board_fr_to_transform(2, 6), black, black_pawn.clone()),
            (board_fr_to_transform(3, 6), black, black_pawn.clone()),
            (board_fr_to_transform(4, 6), black, black_pawn.clone()),
            (board_fr_to_transform(5, 6), black, black_pawn.clone()),
            (board_fr_to_transform(6, 6), black, black_pawn.clone()),
            (board_fr_to_transform(7, 6), black, black_pawn),
            // Rank 8
            (board_fr_to_transform(0, 7), black, black_rook.clone()),
            (board_fr_to_transform(1, 7), black, black_knight.clone()),
            (board_fr_to_transform(2, 7), black, black_bishop.clone()),
            (board_fr_to_transform(3, 7), black, black_queen),
            (board_fr_to_transform(4, 7), black, black_king),
            (board_fr_to_transform(5, 7), black, black_bishop),
            (board_fr_to_transform(6, 7), black, black_knight),
            (board_fr_to_transform(7, 7), black, black_rook),
        ]
        .into_iter()
        .for_each(|child| {
            child_spawner_commands.spawn((child, Pickable::default())); // Each piece is a child of the board, and contains the components `(Transform, Sprite, Piece, Pickable)`.
        });
    });
}
