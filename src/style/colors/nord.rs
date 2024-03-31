use bevy::prelude::*;

use super::ColorScheme;

const NORD_0: Color = Color::rgb(0.18, 0.2, 0.25);
const NORD_1: Color = Color::rgb(0.23, 0.26, 0.32);
const NORD_2: Color = Color::rgb(0.26, 0.3, 0.37);
const NORD_3: Color = Color::rgb(0.3, 0.34, 0.42);

const NORD_4: Color = Color::rgb(0.85, 0.87, 0.91);
const NORD_5: Color = Color::rgb(0.9, 0.91, 0.94);
const NORD_6: Color = Color::rgb(0.93, 0.94, 0.96);

const NORD_7: Color = Color::rgb(0.56, 0.74, 0.73);
const NORD_8: Color = Color::rgb(0.53, 0.75, 0.82);
const NORD_9: Color = Color::rgb(0.51, 0.63, 0.76);
const NORD_10: Color = Color::rgb(0.37, 0.51, 0.67);

const NORD_11: Color = Color::rgb(0.75, 0.38, 0.42);
const NORD_12: Color = Color::rgb(0.82, 0.53, 0.44);
const NORD_13: Color = Color::rgb(0.92, 0.8, 0.55);
const NORD_14: Color = Color::rgb(0.64, 0.75, 0.55);
const NORD_15: Color = Color::rgb(0.71, 0.56, 0.68);

pub struct NordDark;

impl ColorScheme for NordDark {
    const BACKGROUND: Color = NORD_0;
    const BACKGROUND_ALT: Color = NORD_1;
    const FOREGROUND: Color = NORD_4;
    const FOREGROUND_ALT: Color = NORD_5;
    const ACCENT: Color = NORD_8;
    const ACCENT_ALT: Color = NORD_9;

    const TILE_COVERED: Color = NORD_3;
    const TILE_UNCOVERED: Color = NORD_4;
    const TILE_FLAGGED: Color = NORD_12;
    const TILE_MINE: Color = NORD_11;
    const TILE_COUNT: [Color; 8] = [
        NORD_9, NORD_14, NORD_11, NORD_10, NORD_15, NORD_7, NORD_2, NORD_13,
    ];
}

pub struct NordLight;

impl ColorScheme for NordLight {
    const BACKGROUND: Color = NORD_6;
    const BACKGROUND_ALT: Color = NORD_5;
    const FOREGROUND: Color = NORD_0;
    const FOREGROUND_ALT: Color = NORD_1;
    const ACCENT: Color = NORD_8;
    const ACCENT_ALT: Color = NORD_9;
    const TILE_COVERED: Color = NORD_3;
    const TILE_UNCOVERED: Color = NORD_4;
    const TILE_FLAGGED: Color = NORD_12;
    const TILE_MINE: Color = NORD_11;
    const TILE_COUNT: [Color; 8] = [
        NORD_9, NORD_14, NORD_11, NORD_10, NORD_15, NORD_7, NORD_2, NORD_13,
    ];
}
