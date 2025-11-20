use bevy::prelude::*;

use crate::graph::{
    components::Vertex,
    events::VertexRenamedEvent,
    resources::UndoRedoStack,
    undo_redo::{
        RedoAction, RedoVertexRenameEvent, UndoAction, UndoVertexRenameEvent, VertexRenameAction,
    },
};

/// Redoing a vertex rename, creating a new undo action
/// and triggering the main renaming even.
pub fn on_redo_vertex_rename(
    event: On<RedoVertexRenameEvent>,
    mut commands: Commands,
    mut undo_redo: ResMut<UndoRedoStack>,
    mut vertices: Query<&Vertex>,
) {
    let Ok(vertex) = vertices.get_mut(event.entity) else {
        return;
    };
    let current_label = vertex.label.clone();
    undo_redo.push_undo_without_clear(
        UndoAction::UndoRename(VertexRenameAction {
            entity: event.entity,
            name: current_label,
        }),
    );
    commands.trigger(VertexRenamedEvent {
        entity: event.entity,
        new_label: event.name.clone(),
        manual: false,
    });
}

/// Undoing a vertex rename, creating a new redo action,
/// and triggering the main renaming event.
pub fn on_undo_vertex_rename(
    event: On<UndoVertexRenameEvent>,
    mut commands: Commands,
    mut undo_redo: ResMut<UndoRedoStack>,
    mut vertices: Query<&Vertex>,
) {
    let Ok(vertex) = vertices.get_mut(event.entity) else {
        return;
    };
    let current_label = vertex.label.clone();
    undo_redo.push_redo(RedoAction::RedoRename(VertexRenameAction {
        entity: event.entity,
        name: current_label,
    }));
    commands.trigger(VertexRenamedEvent {
        entity: event.entity,
        new_label: event.name.clone(),
        manual: false,
    });
}
