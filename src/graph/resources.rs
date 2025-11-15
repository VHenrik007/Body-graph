use bevy::prelude::*;

/// The currently hovered entity
/// It is used to avoid any checks related to logic
/// like deciding whether to spawn a new vertex or
/// connect to an existing one - that is being hovered over
/// This should be set whenever bevy-picking detects hovering
/// over a vertex or edge. In both cases, separate things
/// might be influenced like the cursor, or the drag-drop outcome.
#[derive(Resource)]
pub struct HoveredEntity(pub Option<Entity>);

/// Used to determine the display features on renaming
/// This can be used both for vertices and edges.
/// TODO: Support multiple renames maybe.
#[derive(Resource, Default, Debug)]
pub struct RenamingState {
    pub active: bool,
    pub entity: Option<Entity>,
    pub temp_text: String,
    pub screen_position: Vec2,
}
