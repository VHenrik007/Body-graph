use bevy::prelude::*;

use crate::graph::{
    components::{Canvas, TemporaryDirectedEdge},
    constants::{BG_COLOR, EDGE_SHAPE, TEMP_EDGE_COLOR},
    observers::*,
};

/// Spawns the canvas containing the graph.
pub fn spawn_canvas(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
) {
    let bg_mesh = meshes.add(Rectangle::from_size(window.size()));
    let bg_material = materials.add(BG_COLOR);

    commands
        .spawn((Canvas, Mesh2d(bg_mesh), MeshMaterial2d(bg_material)))
        .observe(on_bg_clicked);
}

/// Spawns the temporary edge with a `None` source.
pub fn spawn_temporary_edge(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        TemporaryDirectedEdge {
            from: None,
            to: Vec2::ZERO,
        },
        Mesh2d(meshes.add(EDGE_SHAPE)),
        MeshMaterial2d(materials.add(TEMP_EDGE_COLOR)),
    ));
}
