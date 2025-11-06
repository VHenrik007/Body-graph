use bevy::prelude::*;

#[derive(Event)]
pub struct VertexRenameEvent {
    pub entity: Entity,
    pub new_label: String,
}
