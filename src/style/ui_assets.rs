use super::colors::ColorScheme;
use bevy::prelude::*;

#[derive(Resource)]
pub struct UiAssets {
    pub background: Color,
    pub background_alt: Color,
    pub foreground: Color,
    pub foreground_alt: Color,
    pub accent: Color,
    pub accent_alt: Color,
    pub font: Handle<Font>,
}

impl UiAssets {
    pub fn from_colorscheme<T: ColorScheme>() -> Self {
        Self {
            background: T::BACKGROUND,
            background_alt: T::BACKGROUND_ALT,
            foreground: T::FOREGROUND,
            foreground_alt: T::FOREGROUND_ALT,
            accent: T::ACCENT,
            accent_alt: T::ACCENT_ALT,
            font: Default::default(),
        }
    }

    pub fn with_font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }

    pub fn style_title(&self) -> TextStyle {
        TextStyle {
            font_size: 60.,
            color: self.foreground,
            font: self.font.clone(),
        }
    }

    pub fn style_h1(&self) -> TextStyle {
        TextStyle {
            font_size: 40.,
            color: self.foreground,
            font: self.font.clone(),
        }
    }

    pub fn style_h1_accent(&self) -> TextStyle {
        TextStyle {
            font: self.font.clone(),
            font_size: 40.,
            color: self.accent,
        }
    }

    pub fn style_text_accent_alt(&self) -> TextStyle {
        TextStyle {
            font_size: 20.,
            color: self.accent_alt,
            font: self.font.clone(),
        }
    }
}

impl Default for UiAssets {
    fn default() -> Self {
        Self {
            background: Color::BLACK,
            background_alt: Color::DARK_GRAY,
            foreground: Color::WHITE,
            foreground_alt: Color::GRAY,
            accent: Color::RED,
            accent_alt: Color::ORANGE_RED,
            font: Default::default(),
        }
    }
}
