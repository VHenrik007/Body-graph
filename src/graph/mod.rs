use bevy::prelude::*;

mod components;
mod constants;
mod observers;
mod resources;
mod startups;
mod updates;

use resources::HoveredEntity;
use startups::{spawn_canvas, spawn_temporary_edge};
use updates::{project_positions, update_edge_transforms, update_temp_edge_transform};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(HoveredEntity(None))
        .add_systems(Startup, (spawn_canvas, spawn_temporary_edge))
        .add_systems(
            Update,
            (
                project_positions,
                update_edge_transforms,
                update_temp_edge_transform,
            ),
        );
}
