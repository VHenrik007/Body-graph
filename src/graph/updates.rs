use bevy::prelude::*;

use crate::graph::components::{Canvas, DirectedEdge, Position, TemporaryDirectedEdge, Vertex};

/// Using an inner Position component for readability's sake, which is a `Vec2`
/// that needs to be transformed into a proper `Transform`.
pub fn project_positions(mut positionables: Query<(&mut Transform, &Position), Without<Canvas>>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}

/// Each edge should form a segment between its vertices.
pub fn update_edge_transforms(
    edges: Query<(&DirectedEdge, &mut Transform), Without<Vertex>>,
    positions: Query<&Position>,
) {
    for (edge, transform) in edges {
        let Ok(from_pos) = positions.get(edge.from) else {
            continue;
        };
        let Ok(to_pos) = positions.get(edge.to) else {
            continue;
        };

        apply_edge_transform(from_pos.0, to_pos.0, transform.into_inner());
    }
}

/// Each update the temporary edge is either visible or not.
/// Non-visibility is done with 0 scale.
pub fn update_temp_edge_transform(
    edge: Single<(&TemporaryDirectedEdge, &mut Transform)>,
    positions: Query<&Position>,
) {
    let (edge, mut transform) = edge.into_inner();
    let Some(from_vertex) = edge.from else {
        transform.scale.x = 0.0;
        return;
    };

    let Ok(from_pos) = positions.get(from_vertex) else {
        return;
    };

    apply_edge_transform(from_pos.0, edge.to, transform.into_inner());
}

/// Transforms the edge such that it becomes a segment between its two position arguments
fn apply_edge_transform(from_pos: Vec2, to_pos: Vec2, transform: &mut Transform) {
    let direction = to_pos - from_pos;
    let length = direction.length();
    let angle = direction.y.atan2(direction.x);

    transform.translation = from_pos.extend(-0.5);
    transform.rotation = Quat::from_rotation_z(angle);
    transform.scale.x = length;
}
