use bevy::prelude::*;

pub const CONSECUTIVE_CLICK_TIME: f64 = 0.3; // 300 ms

pub const VERTEX_SIZE: f32 = 10.;
pub const VERTEX_SHAPE: Circle = Circle::new(VERTEX_SIZE);
pub const VERTEX_COLOR: Color = Color::srgb(1., 0., 0.);

pub const EDGE_COLOR: Color = Color::srgb(0., 1., 0.);
pub const TEMP_EDGE_COLOR: Color = Color::srgb(0., 0., 1.);
pub const EDGE_SHAPE: Rectangle = Rectangle::from_length(1.0);
pub const EDGE_WIDTH: f32 = 10.;

pub const BG_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);

pub const VERTEX_LABEL_FONT_SIZE: f32 = 12.0;

pub const CANVAS_Z: f32 = -2.0;
pub const EDGE_Z: f32 = -1.0;
pub const VERTEX_Z: f32 = 0.0;
pub const VERTEX_TEXT_Z: f32 = 1.0;
