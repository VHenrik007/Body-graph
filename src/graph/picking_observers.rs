use bevy::prelude::*;

use crate::graph::{
    components::TemporaryDirectedEdge,
    constants::{EDGE_COLOR, HOVERED_EDGE_COLOR, HOVERED_VERTEX_COLOR, VERTEX_COLOR},
    events::{
        CanvasClickedEvent, EdgeClickedEvent, VertexClickedEvent, VertexDragDroppedEvent,
        VertexDraggingEvent,
    },
    helpers::despawn_entity,
    resources::HoveredEntity,
};

/// Clicking the canvas results in a new Vertex.
pub fn on_canvas_clicked(
    click: On<Pointer<Click>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
) {
    let (camera, camera_transform) = camera.into_inner();
    let Ok(world_pos) =
        camera.viewport_to_world_2d(camera_transform, click.pointer_location.position)
    else {
        return;
    };

    commands.trigger(CanvasClickedEvent {
        button: click.button,
        world_position: world_pos,
    });
}

/// If a vertex is hovered we save it into the
/// `HoveredEntity` resource. For more information
/// see the docs at the resource declaration.
pub fn on_vertex_hovered(
    over: On<Pointer<Over>>,
    mut hovered_entity: ResMut<HoveredEntity>,
    mut materials_query: Query<&mut MeshMaterial2d<ColorMaterial>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    hovered_entity.0 = Some(over.entity);
    let new_material = materials.add(HOVERED_VERTEX_COLOR);
    if let Ok(mut material) = materials_query.get_mut(over.entity) {
        material.0 = new_material;
    };
}

/// If no vertex is hovered we make sure to
/// have `None` set for the `HoveredEntity` resource.
///  For more information see the docs at the resource declaration.
pub fn on_vertex_out(
    out: On<Pointer<Out>>,
    mut hovered_entity: ResMut<HoveredEntity>,
    mut materials_query: Query<&mut MeshMaterial2d<ColorMaterial>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    hovered_entity.0 = None;
    let new_material = materials.add(VERTEX_COLOR);
    if let Ok(mut material) = materials_query.get_mut(out.entity) {
        material.0 = new_material;
    };
}

/// If an edge is hovered we save it into the
/// `HoveredEntity` resource. For more information
/// see the docs at the resource declaration.
pub fn on_edge_hovered(
    over: On<Pointer<Over>>,
    mut hovered_entity: ResMut<HoveredEntity>,
    mut materials_query: Query<&mut MeshMaterial2d<ColorMaterial>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    hovered_entity.0 = Some(over.entity);
    let new_material = materials.add(HOVERED_EDGE_COLOR);
    if let Ok(mut material) = materials_query.get_mut(over.entity) {
        material.0 = new_material;
    };
}

/// If no edge is hovered we make sure to
/// have `None` set for the `HoveredEntity` resource.
///  For more information see the docs at the resource declaration.
pub fn on_edge_out(
    out: On<Pointer<Out>>,
    mut hovered_entity: ResMut<HoveredEntity>,
    mut materials_query: Query<&mut MeshMaterial2d<ColorMaterial>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    hovered_entity.0 = None;
    let new_material = materials.add(EDGE_COLOR);
    if let Ok(mut material) = materials_query.get_mut(out.entity) {
        material.0 = new_material;
    };
}

/// Clicking a vertex with the left click twice
/// should start the renaming process.
pub fn on_vertex_clicked(
    click: On<Pointer<Click>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut hovered_entity: ResMut<HoveredEntity>,
    mut commands: Commands,
) {
    let (camera, camera_transform) = camera.into_inner();

    if let Ok(world_pos) =
        camera.viewport_to_world_2d(camera_transform, click.pointer_location.position)
    {
        // Check for deletion first.
        let is_ctrl_held =
            { keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight) };

        if is_ctrl_held {
            despawn_entity(commands, click.entity);
            // For updating the cursor icon
            hovered_entity.0 = None;
            return;
        }

        commands.trigger(VertexClickedEvent {
            entity: click.entity,
            world_position: world_pos,
        });
    }
}

/// Clicking with the right click on a vertex
/// should make the temporary edge visible. In order
/// to use the actual drag delta values, this event is necessary
/// to set the `from` and `to` values.
pub fn on_vertex_dragged(
    drag: On<Pointer<DragStart>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut temp_edge: Single<&mut TemporaryDirectedEdge>,
) {
    if drag.button == PointerButton::Secondary {
        let (camera, camera_transform) = camera.into_inner();

        if let Ok(world_pos) =
            camera.viewport_to_world_2d(camera_transform, drag.pointer_location.position)
        {
            temp_edge.from = Some(drag.entity);
            temp_edge.to = world_pos;
        }
    }
}

/// Dropping the vertex can have two outcomes:
/// - The drop location is occupied by another vertex
/// - The drop location is clear on the Canvas
///
/// Regardless of the outcome, we spawn a new edge, and
/// based on the outcome we also spawn a new vertex.
pub fn on_vertex_drop(
    drag: On<Pointer<DragEnd>>,
    mut commands: Commands,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera.into_inner();

    if let Ok(world_pos) =
        camera.viewport_to_world_2d(camera_transform, drag.pointer_location.position)
    {
        commands.trigger(VertexDragDroppedEvent {
            entity: drag.entity,
            button: drag.button,
            world_position: world_pos,
        });
    };
}

/// Dragging event on a vertex can happen in one of two ways:
/// - Left click -> move the vertex around
/// - Right click -> Attempt to create a connection.
pub fn on_vertex_dragging(
    drag: On<Pointer<Drag>>,
    mut commands: Commands,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera.into_inner();
    if let Ok(world_pos) =
        camera.viewport_to_world_2d(camera_transform, drag.pointer_location.position)
    {
        commands.trigger(VertexDraggingEvent {
            entity: drag.entity,
            button: drag.button,
            world_position: world_pos,
        })
    };
}

/// Handles edge deletion and intermediary vertex creation.
// NOTE: Tried making the vertex creation using `Press` but it
// did not propagate into a vertex drag, so I left it here.
// I might try that again in the future.
pub fn on_edge_clicked(
    click: On<Pointer<Click>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
) {
    let (camera, camera_transform) = camera.into_inner();

    if let Ok(world_pos) =
        camera.viewport_to_world_2d(camera_transform, click.pointer_location.position)
    {
        commands.trigger(EdgeClickedEvent {
            entity: click.entity,
            button: click.button,
            world_position: world_pos,
        });
    };
}
