use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

mod components;
mod constants;
mod events;
mod input;
mod observers;
mod resources;
mod startups;
mod updates;

use input::setup_input;
use resources::{HoveredEntity, RenamingState};
use startups::{spawn_canvas, spawn_temporary_edge};
use updates::{project_positions, update_edge_transforms, update_temp_edge_transform, show_rename_input};
use observers::on_vertex_renamed;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((EnhancedInputPlugin, EguiPlugin::default()))
        .insert_resource(HoveredEntity(None))
        .insert_resource(RenamingState::default())
        .add_observer(on_vertex_renamed)
        .add_systems(Startup, (spawn_canvas, spawn_temporary_edge, setup_input))
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
