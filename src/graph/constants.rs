use bevy::prelude::*;

pub const CONSECUTIVE_CLICK_TIME: f64 = 0.3; // 300 ms

pub const VERTEX_SIZE: f32 = 10.;
pub const VERTEX_SHAPE: Circle = Circle::new(VERTEX_SIZE);
pub const VERTEX_COLOR: Color = Color::srgb(0.3, 0.6, 0.9);
pub const HOVERED_VERTEX_COLOR: Color = Color::srgb(0.4, 0.8, 1.0);

pub const EDGE_COLOR: Color = Color::srgb(0.4, 0.45, 0.5);
pub const TEMP_EDGE_COLOR: Color = Color::srgb(0.95, 0.7, 0.2);
pub const EDGE_SHAPE: Rectangle = Rectangle::from_length(1.0);
pub const EDGE_WIDTH: f32 = 10.;

pub const BG_COLOR: Color = Color::srgb(0.15, 0.15, 0.17);

pub const VERTEX_LABEL_FONT_SIZE: f32 = 12.0;

pub const CANVAS_Z: f32 = -2.0;
pub const EDGE_Z: f32 = -1.0;
pub const VERTEX_Z: f32 = 0.0;
pub const VERTEX_TEXT_Z: f32 = 1.0;

pub const RENAME_CLICK_COUNT: u32 = 2;
