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
mod updates;

use custom_observers::{
    canvas_clicked, click_vertex, edge_clicked, on_vertex_renamed, vertex_drag_dropped,
    vertex_dragging, update_cursor_icon, insert_vertex_on_edge
};
use resources::{HoveredEntity, RenamingState};
use startups::{spawn_canvas, spawn_temporary_edge};
use updates::{
    cursor_icon_manager, project_positions, show_rename_input, update_edge_transforms,
    update_temp_edge_transform,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((EguiPlugin::default(), MeshPickingPlugin))
        .insert_resource(HoveredEntity(None))
        .insert_resource(RenamingState::default())
        .add_observer(on_vertex_renamed)
        .add_observer(canvas_clicked)
        .add_observer(click_vertex)
        .add_observer(vertex_drag_dropped)
        .add_observer(vertex_dragging)
        .add_observer(edge_clicked)
        .add_observer(update_cursor_icon)
        .add_observer(insert_vertex_on_edge)
        .add_systems(Startup, (spawn_canvas, spawn_temporary_edge))
        .add_systems(EguiPrimaryContextPass, show_rename_input)
        .add_systems(
            Update,
            (
                project_positions,
                update_edge_transforms,
                update_temp_edge_transform,
                cursor_icon_manager,
            ),
        );
}
