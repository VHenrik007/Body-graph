use bevy::prelude::*;

use crate::graph::{
    components::{Canvas, DirectedEdge, Position, TemporaryDirectedEdge, Vertex},
    constants::EDGE_WIDTH,
};

/// Using an inner Position component for readability's sake, which is a `Vec2`
/// that needs to be transformed into a proper `Transform`.
pub fn project_positions(mut positionables: Query<(&mut Transform, &Position), Without<Canvas>>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}

/// Each edge should form a segment between its vertices.
pub fn update_edge_transforms(
    mut commands: Commands,
    edges: Query<(&DirectedEdge, &mut Transform, Entity), Without<Vertex>>,
    positions: Query<&Position>,
) {
    for (edge, transform, entity) in edges {
        if let Ok(from_pos) = positions.get(edge.from) {
            if let Ok(to_pos) = positions.get(edge.to) {
                apply_edge_transform(from_pos.0, to_pos.0, transform.into_inner());
                continue;
            };
        };

        // This branch is reached if any of the vertices is missing.
        // This occurs on deleting a vertex, when the edge should be
        // deleted too.
        if let Ok(mut entity) = commands.get_entity(entity) {
            entity.despawn();
        }
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

    transform.translation = (from_pos + direction / 2.0).extend(-0.5);
    transform.rotation = Quat::from_rotation_z(angle);
    transform.scale.x = length;
    transform.scale.y = EDGE_WIDTH;
}
