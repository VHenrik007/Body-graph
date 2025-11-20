use bevy::prelude::*;

use crate::graph::undo_redo::{
    RedoAction, RedoVertexRenameEvent, UndoAction, UndoVertexRenameEvent,
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

#[derive(Resource, Default, Debug)]
pub struct UndoRedoStack {
    pub undo_stack: Vec<UndoAction>,
    pub redo_stack: Vec<RedoAction>,
}

impl UndoRedoStack {
    pub fn push_undo(&mut self, undo_action: UndoAction, is_manual: bool) {
        println!("PUSHING UNDO");
        self.undo_stack.push(undo_action);
        println!("NEW STATE: {:?}", self.undo_stack);
        if is_manual {
            self.redo_stack.clear();
        }
    }

    pub fn undo(&mut self, mut commands: Commands) {
        let Some(undo_action) = self.undo_stack.pop() else {
            println!("Nothing to undo!");
            return;
        };

        match undo_action {
            UndoAction::UndoRename(rename) => {
                commands.trigger(UndoVertexRenameEvent {
                    entity: rename.entity,
                    name: rename.name,
                });
            }
        }
    }

    pub fn push_redo(&mut self, redo_action: RedoAction) {
        println!("PUSHING REDO");
        self.redo_stack.push(redo_action);
        println!("NEW STATE: {:?}", self.redo_stack)
    }

    pub fn redo(&mut self, mut commands: Commands) {
        let Some(redo_action) = self.redo_stack.pop() else {
            println!("Nothing to redo!");
            return;
        };

        match redo_action {
            RedoAction::RedoRename(rename) => {
                commands.trigger(RedoVertexRenameEvent {
                    entity: rename.entity,
                    name: rename.name,
                });
            }
        }
    }
}
