use bevy::prelude::*;

#[derive(Debug)]
pub struct VertexRenameAction {
    pub entity: Entity,
    pub name: String,
}

#[derive(Debug)]
pub enum UndoAction {
    UndoRename(VertexRenameAction),
}

#[derive(Debug)]
pub enum RedoAction {
    RedoRename(VertexRenameAction),
}

#[derive(Event)]
pub struct UndoVertexRenameEvent {
    pub entity: Entity,
    pub name: String,
}

#[derive(Event)]
pub struct RedoVertexRenameEvent {
    pub entity: Entity,
    pub name: String,
}
