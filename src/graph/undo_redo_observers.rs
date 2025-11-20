use bevy::prelude::*;

use crate::graph::{
    bundles::{DirectedEdgeBundle, VertexBundle},
    components::{DirectedEdge, Position, Vertex},
    events::VertexRenamedEvent,
    resources::UndoRedoStack,
    undo_redo::{
        EdgeDeletionAction, EdgeDrawingAction, RedoAction, RedoEdgeDeletionEvent, RedoEdgeDrawingEvent, RedoVertexDeletionEvent, RedoVertexInsertionEvent, RedoVertexMoveEvent, RedoVertexRenameEvent, RedoVertexSpawnEvent, UndoAction, UndoEdgeDeletionEvent, UndoEdgeDrawingEvent, UndoVertexDeletionEvent, UndoVertexInsertionEvent, UndoVertexMoveEvent, UndoVertexRenameEvent, UndoVertexSpawnEvent, VertexDeletionAction, VertexInsertionAction, VertexMoveAction, VertexRenameAction, VertexSpawnAction
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

pub fn on_undo_vertex_move(
    event: On<UndoVertexMoveEvent>,
    mut vertex_positions: Query<&mut Position, With<Vertex>>,
    mut undo_redo: ResMut<UndoRedoStack>,
) {
    let Ok(mut vertex_position) = vertex_positions.get_mut(event.entity) else {
        return;
    };

    let current_position = vertex_position.0;
    vertex_position.0 = event.position;
    undo_redo.push_redo(RedoAction::RedoVertexMoveAction(VertexMoveAction {
        entity: event.entity,
        position: current_position,
    }));
}

pub fn on_redo_vertex_move(
    event: On<RedoVertexMoveEvent>,
    mut vertex_positions: Query<&mut Position, With<Vertex>>,
    mut undo_redo: ResMut<UndoRedoStack>,
) {
    let Ok(mut vertex_position) = vertex_positions.get_mut(event.entity) else {
        return;
    };

    let current_position = vertex_position.0;
    vertex_position.0 = event.position;
    undo_redo.push_undo_without_clear(UndoAction::UndoVertexMoveAction(VertexMoveAction {
        entity: event.entity,
        position: current_position,
    }));
}

pub fn on_redo_edge_draw(
    event: On<RedoEdgeDrawingEvent>,
    mut commands: Commands,
    mut undo_redo: ResMut<UndoRedoStack>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let respawned_edge_id = commands
        .entity(event.action.entity)
        .insert(DirectedEdgeBundle::new(
            event.action.from,
            event.action.to,
            &mut meshes,
            &mut materials,
        ))
        .id();
    DirectedEdgeBundle::add_children(&mut commands, respawned_edge_id);
    undo_redo.push_undo_without_clear(UndoAction::UndoEdgeDrawingAction(EdgeDrawingAction {
        entity: event.action.entity,
        from: event.action.from,
        to: event.action.to,
    }));
}

pub fn on_undo_edge_draw(
    event: On<UndoEdgeDrawingEvent>,
    mut commands: Commands,
    mut undo_redo: ResMut<UndoRedoStack>,
) {
    commands
        .entity(event.action.entity)
        .despawn_children()
        .remove::<DirectedEdgeBundle>();
    undo_redo.push_redo(RedoAction::RedoEdgeDrawingAction(EdgeDrawingAction {
        entity: event.action.entity,
        from: event.action.from,
        to: event.action.to,
    }));
}

pub fn on_undo_edge_deletion(
    event: On<UndoEdgeDeletionEvent>,
    mut commands: Commands,
    mut undo_redo: ResMut<UndoRedoStack>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let respawned_edge_id = commands
        .entity(event.action.entity)
        .insert(DirectedEdgeBundle::new(
            event.action.from,
            event.action.to,
            &mut meshes,
            &mut materials
        ))
        .id();
    DirectedEdgeBundle::add_children(&mut commands, respawned_edge_id);
    undo_redo.push_redo(RedoAction::RedoEdgeDeletionAction(EdgeDeletionAction {
        entity: event.action.entity,
        from: event.action.from,
        to: event.action.to
    }));
}

pub fn on_redo_edge_deletion(
    event: On<RedoEdgeDeletionEvent>,
    mut commands: Commands,
    mut undo_redo: ResMut<UndoRedoStack>,
) {
    commands
        .entity(event.action.entity)
        .despawn_children()
        .remove::<DirectedEdgeBundle>();
    undo_redo.push_undo_without_clear(UndoAction::UndoEdgeDeletionAction(EdgeDeletionAction {
        entity: event.action.entity,
        from: event.action.from,
        to: event.action.to,
    }));
}

pub fn on_undo_vertex_insertion(
    event: On<UndoVertexInsertionEvent>,
    mut commands: Commands,
    mut undo_redo: ResMut<UndoRedoStack>,
    mut edges: Query<&mut DirectedEdge>
) {
    let Ok(mut edge) = edges.get_mut(event.action.edge_entity) else {
        return;
    };

    edge.from = event.action.from;
    edge.to = event.action.to;

    commands
        .entity(event.action.vertex_entity)
        .despawn_children()
        .remove::<VertexBundle>();

    undo_redo.push_redo(RedoAction::RedoVertexInsertionAction(VertexInsertionAction {
        edge_entity: event.action.edge_entity,
        vertex_entity: event.action.vertex_entity,
        vertex_position: event.action.vertex_position,
        from: event.action.from,
        to: event.action.to
    }));
}

pub fn on_redo_vertex_insertion(
    event: On<RedoVertexInsertionEvent>,
    mut commands: Commands,
    mut undo_redo: ResMut<UndoRedoStack>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut edges: Query<&mut DirectedEdge>
) {
    let Ok(mut edge) = edges.get_mut(event.action.edge_entity) else {
        return;
    };

    let respawned_vertex_id = commands
        .entity(event.action.vertex_entity)
        .insert(VertexBundle::new(
            &mut meshes,
            &mut materials,
            event.action.vertex_position,
        ))
        .id();
    VertexBundle::add_children(&mut commands, respawned_vertex_id);

    edge.from = event.action.from;
    edge.to = respawned_vertex_id;

    undo_redo.push_undo_without_clear(UndoAction::UndoVertexInsertionAction(VertexInsertionAction {
        vertex_entity: event.action.vertex_entity,
        vertex_position: event.action.vertex_position,
        edge_entity: event.action.edge_entity,
        from: event.action.from,
        to: event.action.to,
    }));
}