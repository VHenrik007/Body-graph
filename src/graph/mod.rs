use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

mod bundles;
mod components;
mod constants;
mod events;
mod observers;
mod resources;
mod startups;
mod updates;

use observers::on_vertex_renamed;
use resources::{HoveredEntity, RenamingState};
use startups::{spawn_canvas, spawn_temporary_edge};
use updates::{
    project_positions, show_rename_input, update_edge_transforms, update_temp_edge_transform,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((EguiPlugin::default(), MeshPickingPlugin))
        .insert_resource(HoveredEntity(None))
        .insert_resource(RenamingState::default())
        .add_observer(on_vertex_renamed)
        .add_systems(Startup, (spawn_canvas, spawn_temporary_edge))
        .add_systems(EguiPrimaryContextPass, show_rename_input)
        .add_systems(
            Update,
            (
                project_positions,
                update_edge_transforms,
                update_temp_edge_transform,
            ),
        );
}
