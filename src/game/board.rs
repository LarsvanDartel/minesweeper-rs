use bevy::prelude::*;

use super::tilemap::TileMap;

/// Resource to keep track of the game tilemap and handle
/// retrieving tiles
#[derive(Resource)]
pub struct Board {
    pub tile_map: TileMap,
    pub position: Vec2,
    pub size: Vec2,
    pub tile_size: f32,
    pub tile_padding: f32,
}

impl Board {
    /// Translate a mouse position to a tile position
    pub fn mouse_to_tile(&self, window: &Window, mouse_position: Vec2) -> Option<UVec2> {
        let window_size = Vec2::new(window.width(), window.height());
        let mouse_position = mouse_position - window_size / 2.;

        if !self.in_bounds(mouse_position) {
            return None;
        }

        let board_position = mouse_position - self.position;
        let tile_position = (board_position / (self.tile_size + self.tile_padding)).as_uvec2();
        Some(tile_position)
    }

    /// Check if a position is within the bounds of the board
    fn in_bounds(&self, position: Vec2) -> bool {
        position.x >= self.position.x
            && position.x <= self.position.x + self.size.x
            && position.y >= self.position.y
            && position.y <= self.position.y + self.size.y
    }

    /// Checks if all non-bomb tiles have been revealed
    /// used to check if a game is finished
    pub fn all_revealed(&self) -> bool {
        self.tile_map
            .iter()
            .all(|tile| tile.cover.is_none() || tile.is_bomb())
    }
}
