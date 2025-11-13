use bevy::prelude::*;

use crate::graph::components::Position;

/// Despawning an entity requires it to be an entity command
/// first, which I just decided to put here.
pub fn despawn_entity(mut commands: Commands, entity: Entity) {
    if let Ok(mut entity) = commands.get_entity(entity) {
        entity.despawn();
    }
}

/// Updating a position for a given entity is done via a query
/// which is a bit repetitive so I put it here.
pub fn update_entity_position(
    mut positions: Query<&mut Position>,
    entity: Entity,
    new_position: Vec2,
) {
    if let Ok(mut position) = positions.get_mut(entity) {
        position.0 = new_position;
    };
}
