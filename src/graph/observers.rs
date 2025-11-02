use bevy::prelude::*;

use crate::graph::{
    components::{DirectedEdge, Position, TemporaryDirectedEdge, Vertex},
    constants::{EDGE_COLOR, EDGE_SHAPE, VERTEX_COLOR, VERTEX_SHAPE},
    resources::HoveredEntity,
};

/// Clicking the canvas results in a new Vertex.
pub fn on_bg_clicked(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    if click.button == PointerButton::Primary {
        let (camera, camera_transform) = camera.into_inner();

        let Ok(world_pos) =
            camera.viewport_to_world_2d(camera_transform, click.pointer_location.position)
        else {
            return;
        };

        commands
            .spawn((
                Vertex,
                Mesh2d(meshes.add(VERTEX_SHAPE)),
                MeshMaterial2d(materials.add(VERTEX_COLOR)),
                Position(world_pos),
            ))
            .observe(on_vertex_clicked)
            .observe(on_vertex_hovered)
            .observe(on_vertex_out)
            .observe(on_vertex_dragging)
            .observe(on_vertex_drop)
            .observe(on_vertex_dragged);
    }
}

/// If a vertex is hovered we save it into the
/// `HoveredEntity` resource. For more information
/// see the docs at the resource declaration.
pub fn on_vertex_hovered(over: On<Pointer<Over>>, mut hovered_entity: ResMut<HoveredEntity>) {
    hovered_entity.0 = Some(over.entity);
}

/// If no vertex is hovered we make sure to
/// have `None` set for the `HoveredEntity` resource.
///  For more information see the docs at the resource declaration.
pub fn on_vertex_out(_out: On<Pointer<Out>>, mut hovered_entity: ResMut<HoveredEntity>) {
    hovered_entity.0 = None;
}

pub fn on_vertex_clicked(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    vertices: Query<Entity, With<Vertex>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let is_ctrl_held =
        { keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight) };

    if is_ctrl_held {
        let Ok(target_vertex) = vertices.get(click.entity) else {
            return;
        };
        commands.entity(target_vertex).despawn();
    }
}

/// Clicking with the right click on a vertex
/// should make the temporary edge visible. In order
/// to use the actual drag delta values, this event is necessary
/// to set the `from` and `to` values.
pub fn on_vertex_dragged(
    drag: On<Pointer<DragStart>>,
    mut temp_edge: Single<&mut TemporaryDirectedEdge>,
) {
    if drag.button == PointerButton::Secondary {
        let Some(click_position) = drag.hit.position else {
            return;
        };

        temp_edge.from = Some(drag.entity);
        temp_edge.to.x = click_position.x;
        temp_edge.to.y = click_position.y;
    }
}

/// Dropping the vertex can have two outcomes:
/// - The drop location is occupied by another vertex
/// - The drop location is clear on the Canvas
/// Regardless of the outcome, we spawn a new edge, and
/// based on the outcome we also spawn a new vertex.
pub fn on_vertex_drop(
    drag: On<Pointer<DragEnd>>,
    mut commands: Commands,
    mut temp_edge: Single<&mut TemporaryDirectedEdge>,
    camera: Single<(&Camera, &GlobalTransform)>,
    hovered: Res<HoveredEntity>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if drag.button == PointerButton::Secondary {
        let to_vertex;
        if let Some(hovered_vertex) = hovered.0 {
            to_vertex = hovered_vertex;
        } else {
            let (camera, camera_transform) = camera.into_inner();

            let Ok(world_pos) =
                camera.viewport_to_world_2d(camera_transform, drag.pointer_location.position)
            else {
                return;
            };

            let new_vertex = commands
                .spawn((
                    Vertex,
                    Mesh2d(meshes.add(VERTEX_SHAPE)),
                    MeshMaterial2d(materials.add(VERTEX_COLOR)),
                    Position(world_pos),
                ))
                .observe(on_vertex_hovered)
                .observe(on_vertex_out)
                .observe(on_vertex_dragging)
                .observe(on_vertex_drop)
                .observe(on_vertex_dragged)
                .observe(on_vertex_clicked)
                .id();

            to_vertex = new_vertex;
        }

        commands
            .spawn((
                DirectedEdge {
                    from: drag.entity,
                    to: to_vertex,
                },
                Mesh2d(meshes.add(EDGE_SHAPE)),
                MeshMaterial2d(materials.add(EDGE_COLOR)),
            ))
            .observe(on_edge_clicked);

        temp_edge.from = None;
    }
}

/// Dragging event on a vertex can happen in one of two ways:
/// - Left click -> move the vertex around
/// - Right click -> Attempt to create a connection.
pub fn on_vertex_dragging(
    drag: On<Pointer<Drag>>,
    mut positions: Query<&mut Position>,
    mut temp_edge: Single<&mut TemporaryDirectedEdge>,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera.into_inner();
    let cursor_screen_pos = drag.pointer_location.position;
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_screen_pos) else {
        return;
    };

    if drag.button == PointerButton::Primary {
        if let Ok(mut position) = positions.get_mut(drag.entity) {
            position.0 = world_pos;
        };
    } else if drag.button == PointerButton::Secondary {
        temp_edge.to = world_pos;
    }
}

/// Handles edge deletion and intermediary vertex creation.
// NOTE: Tried making the vertex creation using `Press` but it
// did not propagate into a vertex drag, so I left it here.
// I might try that again in the future.
fn on_edge_clicked(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    mut edges: Query<&mut DirectedEdge>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    let is_ctrl_held =
        { keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight) };

    if click.button == PointerButton::Primary {
        if is_ctrl_held {
            if let Ok(mut edge) = commands.get_entity(click.entity) {
                edge.despawn();
            }
        } else {
            let (camera, camera_transform) = camera.into_inner();

            let Ok(world_pos) =
                camera.viewport_to_world_2d(camera_transform, click.pointer_location.position)
            else {
                return;
            };

            let new_vertex = commands
                .spawn((
                    Vertex,
                    Mesh2d(meshes.add(VERTEX_SHAPE)),
                    MeshMaterial2d(materials.add(VERTEX_COLOR)),
                    Position(world_pos),
                ))
                .observe(on_vertex_hovered)
                .observe(on_vertex_out)
                .observe(on_vertex_dragging)
                .observe(on_vertex_drop)
                .observe(on_vertex_dragged)
                .observe(on_vertex_clicked)
                .id();

            let Ok(mut edge) = edges.get_mut(click.entity) else {
                return;
            };

            let prev_to = edge.to;
            edge.to = new_vertex;

            commands
                .spawn((
                    DirectedEdge {
                        from: new_vertex,
                        to: prev_to,
                    },
                    Mesh2d(meshes.add(EDGE_SHAPE)),
                    MeshMaterial2d(materials.add(EDGE_COLOR)),
                ))
                .observe(on_edge_clicked);
        }
    }
}
