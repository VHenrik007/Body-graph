use bevy::prelude::*;

pub const VERTEX_SIZE: f32 = 10.;
pub const VERTEX_SHAPE: Circle = Circle::new(VERTEX_SIZE);
pub const VERTEX_COLOR: Color = Color::srgb(1., 0., 0.);

pub const EDGE_COLOR: Color = Color::srgb(0., 1., 0.);
pub const TEMP_EDGE_COLOR: Color = Color::srgb(0., 0., 1.);
//pub const EDGE_SHAPE: Rectangle = Rectangle::from_corners(Vec2::ZERO, Vec2::ONE);
pub const EDGE_SHAPE: Rectangle = Rectangle::from_length(1.0);
pub const EDGE_WIDTH: f32 = 10.;

pub const BG_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
