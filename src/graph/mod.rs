use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

mod components;
mod constants;
mod input;
mod observers;
mod resources;
mod startups;
mod updates;

use input::setup_input;
use resources::HoveredEntity;
use startups::{spawn_canvas, spawn_temporary_edge};
use updates::{project_positions, update_edge_transforms, update_temp_edge_transform};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(EnhancedInputPlugin)
        .insert_resource(HoveredEntity(None))
        .add_systems(Startup, (spawn_canvas, spawn_temporary_edge, setup_input))
        .add_systems(
            Update,
            (
                project_positions,
                update_edge_transforms,
                update_temp_edge_transform,
            ),
        );
}
