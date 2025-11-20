use bevy::prelude::*;

use crate::graph::{
    bundles::VertexBundle,
    components::Vertex,
    events::VertexRenamedEvent,
    resources::UndoRedoStack,
    undo_redo::{
        RedoAction, RedoVertexDeletionEvent, RedoVertexRenameEvent, RedoVertexSpawnEvent,
        UndoAction, UndoVertexDeletionEvent, UndoVertexRenameEvent, UndoVertexSpawnEvent,
        VertexDeletionAction, VertexRenameAction, VertexSpawnAction,
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
    undo_redo.push_undo_without_clear(UndoAction::UndoVertexRename(VertexRenameAction {
        entity: event.entity,
        name: current_label,
    }));
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
    undo_redo.push_redo(RedoAction::RedoVertexRename(VertexRenameAction {
        entity: event.entity,
        name: current_label,
    }));
    commands.trigger(VertexRenamedEvent {
        entity: event.entity,
        new_label: event.name.clone(),
        manual: false,
    });
}

pub fn on_undo_vertex_deletion(
    event: On<UndoVertexDeletionEvent>,
    mut commands: Commands,
    mut undo_redo: ResMut<UndoRedoStack>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let respawned_vertex_id = commands
        .entity(event.entity)
        .insert(VertexBundle::new_with_label(
            &mut meshes,
            &mut materials,
            event.position,
            &event.vertex_label,
        ))
        .id();
    VertexBundle::add_children_with_label(&mut commands, respawned_vertex_id, &event.vertex_label);
    undo_redo.push_redo(RedoAction::RedoVertexDeletion(VertexDeletionAction {
        entity: event.entity,
        position: event.position,
        vertex_label: event.vertex_label.clone(),
    }));
}

pub fn on_redo_vertex_deletion(
    event: On<RedoVertexDeletionEvent>,
    mut commands: Commands,
    mut undo_redo: ResMut<UndoRedoStack>,
) {
    commands
        .entity(event.entity)
        .despawn_children()
        .remove::<VertexBundle>();
    undo_redo.push_undo_without_clear(UndoAction::UndoVertexDeletion(VertexDeletionAction {
        entity: event.entity,
        position: event.position,
        vertex_label: event.vertex_label.clone(),
    }));
}

pub fn on_undo_vertex_spawn(
    event: On<UndoVertexSpawnEvent>,
    mut commands: Commands,
    mut undo_redo: ResMut<UndoRedoStack>,
) {
    commands
        .entity(event.entity)
        .despawn_children()
        .remove::<VertexBundle>();
    undo_redo.push_redo(RedoAction::RedoVertexSpawn(VertexSpawnAction {
        entity: event.entity,
        position: event.position,
    }));
}

pub fn on_redo_vertex_spawn(
    event: On<RedoVertexSpawnEvent>,
    mut commands: Commands,
    mut undo_redo: ResMut<UndoRedoStack>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let respawned_vertex_id = commands
        .entity(event.entity)
        .insert(VertexBundle::new(
            &mut meshes,
            &mut materials,
            event.position,
        ))
        .id();
    VertexBundle::add_children(&mut commands, respawned_vertex_id);
    undo_redo.push_undo_without_clear(UndoAction::UndoVertexSpawn(VertexSpawnAction {
        entity: event.entity,
        position: event.position,
    }));
}
