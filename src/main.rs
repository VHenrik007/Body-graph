use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig},
    prelude::*,
};

mod graph;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn get_fps_overlay_plugin_config() -> FpsOverlayPlugin {
    FpsOverlayPlugin {
        config: FpsOverlayConfig {
            text_config: TextFont {
                font_size: 42.0,
                ..default()
            },
            text_color: Color::srgb(0.0, 1.0, 0.0),
            refresh_interval: core::time::Duration::from_millis(100),
            enabled: true,
            frame_time_graph_config: FrameTimeGraphConfig {
                enabled: true,
                min_fps: 60.0,
                target_fps: 240.0,
            },
        },
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            graph::plugin,
            // get_fps_overlay_plugin_config(),
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}
