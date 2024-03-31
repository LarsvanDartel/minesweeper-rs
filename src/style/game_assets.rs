use super::colors::ColorScheme;
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAssets {
    pub board: Color,
    pub tile_covered: Color,
    pub tile_uncovered: Color,
    pub tile_flagged: Color,
    pub tile_mine: Color,
    pub tile_count: [Color; 8],
    pub tile_count_font: Handle<Font>,
}

impl GameAssets {
    pub fn from_colorscheme<T: ColorScheme>() -> Self {
        Self {
            board: T::BACKGROUND,
            tile_covered: T::TILE_COVERED,
            tile_uncovered: T::TILE_UNCOVERED,
            tile_flagged: T::TILE_FLAGGED,
            tile_mine: T::TILE_MINE,
            tile_count: T::TILE_COUNT,
            tile_count_font: Default::default(),
        }
    }

    pub fn with_font(mut self, font: Handle<Font>) -> Self {
        self.tile_count_font = font;
        self
    }

    pub fn count_color(&self, count: usize) -> Color {
        let count = count.saturating_sub(1).min(7);
        self.tile_count[count]
    }
}

impl Default for GameAssets {
    fn default() -> Self {
        Self {
            board: Color::WHITE,
            tile_covered: Color::DARK_GRAY,
            tile_uncovered: Color::GRAY,
            tile_flagged: Color::RED,
            tile_mine: Color::RED,
            tile_count: [
                Color::BLUE,
                Color::GREEN,
                Color::RED,
                Color::PURPLE,
                Color::CRIMSON,
                Color::CYAN,
                Color::BLACK,
                Color::DARK_GRAY,
            ],
            tile_count_font: Default::default(),
        }
    }
}
