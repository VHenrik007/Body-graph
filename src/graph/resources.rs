use bevy::prelude::*;

use crate::graph::{
    helpers::despawn_entity,
    undo_redo::{
        RedoAction, RedoVertexDeletionEvent, RedoVertexRenameEvent, RedoVertexSpawnEvent, UndoAction, UndoVertexDeletionEvent, UndoVertexRenameEvent, UndoVertexSpawnEvent
    },
};

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

/// The stack resource for managing undo/redo operations
/// Contains redo and undo separately, and pushes/pops
/// fron one onto/from another.
#[derive(Resource, Debug)]
pub struct UndoRedoStack {
    max_size: usize,
    /// Undo actions
    pub undo_stack: Vec<UndoAction>,
    /// Redo actions
    pub redo_stack: Vec<RedoAction>,
}

impl Default for UndoRedoStack {
    fn default() -> Self {
        UndoRedoStack {
            max_size: 64,
            undo_stack: Vec::default(),
            redo_stack: Vec::default(),
        }
    }
}

impl UndoRedoStack {
    /// Pushing an undo also clears redo as a new action after multiple undo
    /// operations might invalidate a redo in the stack.
    pub fn push_undo(&mut self, undo_action: UndoAction, commands: &mut Commands) {
        println!("+ UNDO: {:?}", undo_action);

        if self.undo_stack.len() == self.max_size {
            let action = self.undo_stack.remove(0);
            match action {
                UndoAction::UndoVertexDeletion(deletion_action) => {
                    despawn_entity(commands, deletion_action.entity);
                }
                _ => {}
            }
        }
        self.undo_stack.push(undo_action);

        for redo_action in self.redo_stack.iter_mut() {
            match redo_action {
                RedoAction::RedoVertexSpawn(spawn_action) => {
                    despawn_entity(commands, spawn_action.entity);
                }
                _ => {}
            }
        }

        self.redo_stack.clear();
    }

    /// This push does not clear the redo stack. It's used only for redo actions,
    /// as switching back&forth between redo and undo is okay.
    pub fn push_undo_without_clear(&mut self, undo_action: UndoAction) {
        println!("+ UNDO w/o c.: {:?}", undo_action);
        self.undo_stack.push(undo_action);
    }

    /// Undo the latest operation by matching the enum and triggering
    /// the corresponding event.
    pub fn undo(&mut self, mut commands: Commands) {
        let Some(undo_action) = self.undo_stack.pop() else {
            return;
        };

        println!("- UNDO: {:?}", undo_action);

        match undo_action {
            UndoAction::UndoVertexRename(rename) => {
                commands.trigger(UndoVertexRenameEvent {
                    entity: rename.entity,
                    name: rename.name,
                });
            }
            UndoAction::UndoVertexDeletion(deletion) => {
                commands.trigger(UndoVertexDeletionEvent {
                    entity: deletion.entity,
                    position: deletion.position,
                    vertex_label: deletion.vertex_label,
                });
            }
            UndoAction::UndoVertexSpawn(spawn) => {
                commands.trigger(UndoVertexSpawnEvent {
                    entity: spawn.entity,
                    position: spawn.position,
                });
            }
        }
    }

    /// Pushing a redo action. The size check is unnecessary
    /// as all elements come from the undo stack, and the redo
    /// stack is cleared on a new user action.
    pub fn push_redo(&mut self, redo_action: RedoAction) {
        println!("+ REDO: {:?}", redo_action);
        self.redo_stack.push(redo_action);
    }

    /// Redo the latest operation by matching the enum and triggering
    /// the corresponding event.
    pub fn redo(&mut self, mut commands: Commands) {
        let Some(redo_action) = self.redo_stack.pop() else {
            return;
        };

        println!("- REDO: {:?}", redo_action);

        match redo_action {
            RedoAction::RedoVertexRename(rename) => {
                commands.trigger(RedoVertexRenameEvent {
                    entity: rename.entity,
                    name: rename.name,
                });
            },
            RedoAction::RedoVertexDeletion(deletion) => {
                commands.trigger(RedoVertexDeletionEvent {
                    entity: deletion.entity,
                    position: deletion.position,
                    vertex_label: deletion.vertex_label,
                });
            },
            RedoAction::RedoVertexSpawn(spawn) => {
                commands.trigger(RedoVertexSpawnEvent {
                    entity: spawn.entity,
                    position: spawn.position,
                });
            }
        }
    }
}
