use bevy::prelude::*;

use crate::graph::{
    bundles::{DirectedEdgeBundle, VertexBundle},
    components::{ClickTracker, DirectedEdge, Position, TemporaryDirectedEdge, Vertex},
    constants::{CONSECUTIVE_CLICK_TIME, RENAME_CLICK_COUNT},
    events::{
        CanvasClickedEvent, EdgeClickedEvent, UpdateCursorIconEvent, VertexClickedEvent,
        VertexDragDroppedEvent, VertexDraggingEvent, VertexRenamedEvent,
    },
    helpers::{despawn_entity, update_entity_position},
    resources::{HoveredEntity, RenamingState, UndoRedoStack},
    undo_redo::{EdgeDrawingAction, UndoAction, VertexRenameAction, VertexSpawnAction},
};

/// When a vertex is renamed, we update the label and
/// the text component that is a child of the vertex entity.
pub fn on_vertex_renamed(
    event: On<VertexRenamedEvent>,
    mut vertex_query: Query<(&mut Vertex, &Children)>,
    mut text_query: Query<&mut Text2d>,
    mut undo_redo: ResMut<UndoRedoStack>,
    mut commands: Commands,
) {
    let new_label = event.new_label.clone();
    let vertex_entity = event.entity;

    let Ok((mut renamed_vertex, children)) = vertex_query.get_mut(vertex_entity) else {
        return;
    };
    let old_label = renamed_vertex.label.clone();
    renamed_vertex.label = new_label.clone();

    for child in children.iter() {
        let Ok(mut modified_text) = text_query.get_mut(child) else {
            continue;
        };

        modified_text.0 = new_label.clone();
    }

    if event.manual {
        undo_redo.push_undo(
            UndoAction::UndoVertexRename(VertexRenameAction {
                entity: event.entity,
                name: old_label,
            }),
            &mut commands,
        );
    }
}

/// Clicking a vertex could mean deletion,
/// or renaming.
pub fn click_vertex(
    vertex_click: On<VertexClickedEvent>,
    mut trackers: Query<&mut ClickTracker, With<Vertex>>,
    vertices: Query<(Entity, &Vertex)>,
    time: Res<Time>,
    renaming: ResMut<RenamingState>,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    let Ok(mut tracker) = trackers.get_mut(vertex_click.entity) else {
        return;
    };

    let current_time = time.elapsed_secs_f64();
    let Some(last_time) = tracker.last_click_time else {
        tracker.click_count = 1;
        tracker.last_click_time = Some(current_time);
        return;
    };

    if current_time - last_time <= CONSECUTIVE_CLICK_TIME {
        tracker.click_count += 1;
        tracker.last_click_time = Some(current_time);
        if tracker.click_count == RENAME_CLICK_COUNT {
            if let Ok((_, target_vertex)) = vertices.get(vertex_click.entity) {
                activate_vertex_renaming(
                    vertex_click.entity,
                    target_vertex,
                    renaming,
                    vertex_click.world_position,
                    camera,
                );
            };
        }
    } else {
        tracker.click_count = 1;
        tracker.last_click_time = Some(current_time);
    }
}

/// Since almost all clicks and interactions use
/// world positions, this function puts it back to
/// the screen position as well.
fn activate_vertex_renaming(
    vertex_entity: Entity,
    vertex_component: &Vertex,
    mut renaming: ResMut<RenamingState>,
    world_position: Vec2,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    // This is a bit sad, as the reason I
    // have my custom events in the
    // first place is to not have the world position
    // conversions all over the place, and now here I am
    // putting it back to screen space.
    let (camera, camera_transform) = camera.into_inner();
    if let Ok(screen_pos) = camera.world_to_viewport(camera_transform, world_position.extend(0.0)) {
        renaming.active = true;
        renaming.entity = Some(vertex_entity);
        renaming.temp_text = vertex_component.label.clone();
        renaming.screen_position = screen_pos;
    }
}

/// Left clicking the canvas spawns
/// a new vertex
// TODO right click context menu maybe.
pub fn canvas_clicked(
    click: On<CanvasClickedEvent>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut undo_redo: ResMut<UndoRedoStack>,
) {
    if click.button == PointerButton::Primary {
        let entity_id = VertexBundle::spawn(
            &mut commands,
            meshes.into_inner(),
            materials.into_inner(),
            click.world_position,
        );
        undo_redo.push_undo(
            UndoAction::UndoVertexSpawn(VertexSpawnAction {
                entity: entity_id,
                position: click.world_position,
            }),
            &mut commands,
        );
    }
}

/// Dropping a vertex drag can mean two things
/// depending on which button started the dragging.
/// - Left: Move the vertex around.
/// - Right: Draw an edge to a new vertex at cursor location.
pub fn vertex_drag_dropped(
    drag: On<VertexDragDroppedEvent>,
    hovered: Res<HoveredEntity>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    vertices: Query<Entity, With<Vertex>>,
    edge_entities: Query<Entity, With<DirectedEdge>>,
    edges: Query<&mut DirectedEdge>,
    mut commands: Commands,
    mut temp_edge: Single<&mut TemporaryDirectedEdge>,
    mut undo_redo: ResMut<UndoRedoStack>,
) {
    let materials = materials.into_inner();
    let meshes = meshes.into_inner();
    if drag.button == PointerButton::Secondary {
        let to_entity;
        if let Some(hovered_entity) = hovered.0 {
            if let Ok(hovered_vertex) = vertices.get(hovered_entity) {
                to_entity = hovered_vertex;
                let edge_entity = DirectedEdgeBundle::spawn(
                    drag.entity,
                    to_entity,
                    &mut commands,
                    meshes,
                    materials,
                );

                undo_redo.push_undo(
                    UndoAction::UndoEdgeDrawingAction(EdgeDrawingAction {
                        entity: edge_entity,
                        from: drag.entity,
                        to: to_entity,
                    }),
                    &mut commands,
                );

                temp_edge.from = None;
                return;
            } else if let Ok(hovered_edge) = edge_entities.get(hovered_entity) {
                if let Some(inserted_vertex) = insert_vertex_on_edge(
                    meshes,
                    materials,
                    edges,
                    &mut commands,
                    drag.world_position,
                    hovered_edge,
                ) {
                    to_entity = inserted_vertex;
                } else {
                    return;
                };
            } else {
                return;
            }
        } else {
            to_entity = VertexBundle::spawn(&mut commands, meshes, materials, drag.world_position);
            undo_redo.push_undo(
                UndoAction::UndoVertexSpawn(VertexSpawnAction {
                    entity: to_entity,
                    position: drag.world_position,
                }),
                &mut commands,
            );
        }
        DirectedEdgeBundle::spawn(drag.entity, to_entity, &mut commands, meshes, materials);
        temp_edge.from = None;
    }
}

/// Clicking on an edge can either delete it
/// or insert a new vertex.
pub fn edge_clicked(
    click: On<EdgeClickedEvent>,
    mut commands: Commands,
    mut hovered_entity: ResMut<HoveredEntity>,
    edges: Query<&mut DirectedEdge>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let is_ctrl_held =
        { keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight) };

    if is_ctrl_held {
        despawn_entity(&mut commands, click.entity);
        // For updating the cursor icon
        hovered_entity.0 = None;
        return;
    }

    if click.button == PointerButton::Primary {
        insert_vertex_on_edge(
            meshes.into_inner(),
            materials.into_inner(),
            edges,
            &mut commands,
            click.world_position,
            click.entity,
        );
    }
}

/// Inserting a vertex on an edge involves
/// creating a new vertex, setting that as the
/// `to` for the edge, and creating a new one
/// between the new and previous vertices.
pub fn insert_vertex_on_edge(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    mut edges: Query<&mut DirectedEdge>,
    commands: &mut Commands,
    world_position: Vec2,
    entity: Entity,
) -> Option<Entity> {
    let Ok(mut edge) = edges.get_mut(entity) else {
        return None;
    };

    let new_vertex = VertexBundle::spawn(commands, meshes, materials, world_position);

    let prev_to = edge.to;
    edge.to = new_vertex;

    DirectedEdgeBundle::spawn(new_vertex, prev_to, commands, meshes, materials);

    Some(new_vertex)
}

/// Draggin a vertex either repositions
/// it or updates the temporary edge being
/// drawn from it.
pub fn vertex_dragging(
    drag: On<VertexDraggingEvent>,
    positions: Query<&mut Position>,
    mut temp_edge: Single<&mut TemporaryDirectedEdge>,
    mut renaming: ResMut<RenamingState>,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    if drag.button == PointerButton::Primary {
        if let Some(renaming_entity) = renaming.entity {
            if renaming_entity == drag.entity {
                let (camera, camera_transform) = camera.into_inner();
                if let Ok(screen_pos) =
                    camera.world_to_viewport(camera_transform, drag.world_position.extend(0.0))
                {
                    renaming.screen_position = screen_pos;
                }
            }
        }
        update_entity_position(positions, drag.entity, drag.world_position);
    } else if drag.button == PointerButton::Secondary {
        temp_edge.to = drag.world_position;
    }
}

/// Updating the cursor icon is done by inserting it
/// as a new component.
pub fn update_cursor_icon(
    event: On<UpdateCursorIconEvent>,
    mut commands: Commands,
    window: Single<Entity, With<Window>>,
) {
    commands
        .entity(window.entity())
        .insert(event.new_icon.clone());
}
