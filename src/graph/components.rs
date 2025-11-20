use bevy::prelude::*;

use crate::graph::constants::CANVAS_Z;

/// Custom position for better readability
/// and in-game position representation
#[derive(Component, Default, Clone, Debug)]
#[require(Transform)]
pub struct Position(pub Vec2);

/// A vertex in the graph
#[derive(Component, Default, Debug)]
#[require(Position)]
pub struct Vertex {
    pub label: String,
}

/// An edge in the graph
#[derive(Component)]
pub struct DirectedEdge {
    pub from: Entity,
    pub to: Entity,
}

/// The edge that should be drawn
/// between a "dragged" vertex and
/// the cursor.
#[derive(Component)]
pub struct TemporaryDirectedEdge {
    pub from: Option<Entity>,
    pub to: Vec2,
}

/// The canvas of the graph.
/// This surface captures the non-vertex left click.
#[derive(Component)]
#[require(
    Transform = Transform {
        translation: Vec3 { x: 0., y: 0., z: CANVAS_Z },
        ..Default::default()
    }
)]
pub struct Canvas;

/// Used for deciding for double (or any other) clicking
/// events.
#[derive(Component, Default, Debug)]
pub struct ClickTracker {
    pub last_click_time: Option<f64>,
    pub click_count: u32,
}
