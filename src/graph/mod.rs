use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

mod bundles;
mod components;
mod constants;
mod custom_observers;
mod events;
mod helpers;
mod picking_observers;
mod resources;
mod startups;
mod undo_redo;
mod undo_redo_observers;
mod updates;

use custom_observers::{
    canvas_clicked, click_vertex, edge_clicked, on_vertex_renamed, update_cursor_icon,
    vertex_drag_dropped, vertex_dragging,
};
use resources::{HoveredEntity, RenamingState, UndoRedoStack};
use startups::{spawn_canvas, spawn_temporary_edge};
use undo_redo_observers::{
    on_redo_edge_draw, on_redo_vertex_deletion, on_redo_vertex_move, on_redo_vertex_rename,
    on_redo_vertex_spawn, on_undo_edge_draw, on_undo_vertex_deletion, on_undo_vertex_move,
    on_undo_vertex_rename, on_undo_vertex_spawn, on_undo_edge_deletion, on_redo_edge_deletion,
    on_redo_vertex_insertion, on_undo_vertex_insertion
};
use updates::{
    cursor_icon_manager, project_positions, show_rename_input,
    undo_redo_system, update_edge_transforms, update_temp_edge_transform,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((EguiPlugin::default(), MeshPickingPlugin))
        .insert_resource(HoveredEntity(None))
        .insert_resource(RenamingState::default())
        .insert_resource(UndoRedoStack::default())
        .add_observer(on_vertex_renamed)
        .add_observer(canvas_clicked)
        .add_observer(click_vertex)
        .add_observer(vertex_drag_dropped)
        .add_observer(vertex_dragging)
        .add_observer(edge_clicked)
        .add_observer(update_cursor_icon)
        .add_observer(on_redo_vertex_rename)
        .add_observer(on_undo_vertex_rename)
        .add_observer(on_undo_vertex_deletion)
        .add_observer(on_redo_vertex_deletion)
        .add_observer(on_undo_vertex_spawn)
        .add_observer(on_redo_vertex_spawn)
        .add_observer(on_redo_vertex_move)
        .add_observer(on_undo_vertex_move)
        .add_observer(on_undo_edge_draw)
        .add_observer(on_redo_edge_draw)
        .add_observer(on_undo_edge_deletion)
        .add_observer(on_redo_edge_deletion)
        .add_observer(on_undo_vertex_insertion)
        .add_observer(on_redo_vertex_insertion)
        .add_systems(Startup, (spawn_canvas, spawn_temporary_edge))
        .add_systems(EguiPrimaryContextPass, show_rename_input)
        .add_systems(
            Update,
            (
                project_positions,
                update_edge_transforms,
                update_temp_edge_transform,
                cursor_icon_manager,
                undo_redo_system,
            ),
        );
}
