use bevy::prelude::*;

use crate::graph::{
    components::Vertex,
    events::VertexRenamedEvent,
    resources::UndoRedoStack,
    undo_redo::{
        RedoAction, RedoVertexRenameEvent, UndoAction, UndoVertexRenameEvent, VertexRenameAction,
    },
};

pub fn on_redo_vertex_rename(
    event: On<RedoVertexRenameEvent>,
    mut commands: Commands,
    mut undo_redo: ResMut<UndoRedoStack>,
    mut vertices: Query<&Vertex>,
) {
    println!("REDOING RENAME: event string: {:?}", event.name);
    let Ok(vertex) = vertices.get_mut(event.entity) else {
        return;
    };
    let current_label = vertex.label.clone();
    undo_redo.push_undo(
        UndoAction::UndoRename(VertexRenameAction {
            entity: event.entity,
            name: current_label,
        }),
        false,
    );
    commands.trigger(VertexRenamedEvent {
        entity: event.entity,
        new_label: event.name.clone(),
        manual: false,
    });
}

pub fn on_undo_vertex_rename(
    event: On<UndoVertexRenameEvent>,
    mut commands: Commands,
    mut undo_redo: ResMut<UndoRedoStack>,
    mut vertices: Query<&Vertex>,
) {
    println!("UNDOING RENAME: event string: {:?}", event.name);
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
