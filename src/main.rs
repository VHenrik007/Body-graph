use bevy::prelude::*;

mod graph;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin, graph::plugin))
        .add_systems(Startup, spawn_camera)
        .run();
}
