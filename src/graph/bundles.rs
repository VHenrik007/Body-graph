use bevy::prelude::*;

use crate::graph::{
    components::{Vertex, ClickTracker, Position},
    constants::{VERTEX_LABEL_FONT_SIZE, VERTEX_SHAPE, VERTEX_COLOR},
    observers::{
        on_vertex_clicked,
        on_vertex_hovered,
        on_vertex_out,
        on_vertex_dragged,
        on_vertex_dragging,
        on_vertex_drop
    }
};

#[derive(Bundle)]
pub struct VertexBundle {
    vertex: Vertex,
    click_tracker: ClickTracker,
    text: Text2d,
    font: TextFont,
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    position: Position,
}

impl VertexBundle {
    pub fn new(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        position: Vec2,
    ) -> Self {
        Self {
            vertex: Vertex::default(),
            click_tracker: ClickTracker::default(),
            text: Text2d::default(),
            font: TextFont::from_font_size(VERTEX_LABEL_FONT_SIZE),
            mesh: Mesh2d(meshes.add(VERTEX_SHAPE)),
            material: MeshMaterial2d(materials.add(VERTEX_COLOR)),
            position: Position(position),
        }
    }

    pub fn observe_vertex(entity: &mut EntityCommands) {
        entity
            .observe(on_vertex_clicked)
            .observe(on_vertex_hovered)
            .observe(on_vertex_out)
            .observe(on_vertex_dragging)
            .observe(on_vertex_drop)
            .observe(on_vertex_dragged);
    }
}