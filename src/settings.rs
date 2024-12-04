use bevy::prelude::*;
use bevy::color::palettes::css;

#[derive(Resource)]
pub struct Settings {
    pub layers: u32,
    pub color_up: Color,
    pub color_down: Color,
    pub color_left: Color,
    pub color_right: Color,
    pub color_front: Color,
    pub color_back: Color,
    pub color_beleved: Color,

    pub view_rotation_speed: f32,
    pub layer_rotation_speed: f32,
    pub rotation_trigger_value: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            layers: 3,
            color_down: Color::Srgba(css::WHITE),
            color_front: Color::Srgba(css::GREEN),
            color_up:  Color::Srgba(css::YELLOW),
            color_left: Color::Srgba(css::RED),
            color_back: Color::Srgba(css::BLUE),
            color_right: Color::Srgba(css::ORANGE),
            color_beleved: Color::Srgba(css::BLACK),

            view_rotation_speed: 5.0,
            layer_rotation_speed: 5.0,
            rotation_trigger_value: 0.8,
        }
    }
}
