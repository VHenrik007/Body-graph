use bevy::prelude::*;

// Three main parts:
// - Actions
// - Undo/Redo enums containing actions
// - Events

// Actions
// Actions are arbitrary actions performed during the program
// by the user. They contain all the necessary info around the action

#[derive(Debug)]
pub struct VertexRenameAction {
    pub entity: Entity,
    pub name: String,
}

#[derive(Debug)]
pub struct VertexDeletionAction {
    pub entity: Entity,
    pub position: Vec2,
    pub vertex_label: String,
}

#[derive(Debug)]
pub struct VertexSpawnAction {
    pub entity: Entity,
    pub position: Vec2,
}

#[derive(Debug)]
pub struct VertexMoveAction {
    pub entity: Entity,
    pub position: Vec2,
}

// Undo/Redo enums
// These enums contain some action variant are they themselves are
// stored in the `UndoRedoStack` stacks.

#[derive(Debug)]
pub enum UndoAction {
    UndoVertexRename(VertexRenameAction),
    UndoVertexDeletion(VertexDeletionAction),
    UndoVertexSpawn(VertexSpawnAction),
    UndoVertexMoveAction(VertexMoveAction),
}

#[derive(Debug)]
pub enum RedoAction {
    RedoVertexRename(VertexRenameAction),
    RedoVertexDeletion(VertexDeletionAction),
    RedoVertexSpawn(VertexSpawnAction),
    RedoVertexMoveAction(VertexMoveAction),
}

// Events
// The particular events raised when an action is undone/redone.
// Contains information necessary for that particular case, originated
// from the main action type.
// TODO: This should be simplified to only contain the action type instead
//       of individual fields.

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

#[derive(Event)]
pub struct UndoVertexDeletionEvent {
    pub entity: Entity,
    pub position: Vec2,
    pub vertex_label: String,
}

#[derive(Event)]
pub struct RedoVertexDeletionEvent {
    pub entity: Entity,
    pub position: Vec2,
    pub vertex_label: String,
}

#[derive(Event)]
pub struct UndoVertexSpawnEvent {
    pub entity: Entity,
    pub position: Vec2,
}

#[derive(Event)]
pub struct RedoVertexSpawnEvent {
    pub entity: Entity,
    pub position: Vec2,
}

#[derive(Event)]
pub struct UndoVertexMoveEvent {
    pub entity: Entity,
    pub position: Vec2,
}

#[derive(Event)]
pub struct RedoVertexMoveEvent {
    pub entity: Entity,
    pub position: Vec2,
}
