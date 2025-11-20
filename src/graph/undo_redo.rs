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

// Undo/Redo enums
// These enums contain some action variant are they themselves are
// stored in the `UndoRedoStack` stacks.

#[derive(Debug)]
pub enum UndoAction {
    UndoRename(VertexRenameAction),
}

#[derive(Debug)]
pub enum RedoAction {
    RedoRename(VertexRenameAction),
}

// Events
// The particular events raised when an action is undone/redone.
// Contains information necessary for that particular case, originated
// from the main action type.

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
