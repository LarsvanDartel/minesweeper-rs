use bevy::prelude::*;

mod nord;
pub use nord::{NordDark, NordLight};

pub trait ColorScheme {
    const BACKGROUND: Color;
    const BACKGROUND_ALT: Color;
    const FOREGROUND: Color;
    const FOREGROUND_ALT: Color;
    const ACCENT: Color;
    const ACCENT_ALT: Color;

    const TILE_COVERED: Color;
    const TILE_UNCOVERED: Color;
    const TILE_FLAGGED: Color;
    const TILE_MINE: Color;
    const TILE_COUNT: [Color; 8];
}
