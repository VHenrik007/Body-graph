use bevy::prelude::*;

use crate::graph::{
    components::{ClickTracker, Position, Vertex},
    constants::{VERTEX_COLOR, VERTEX_LABEL_FONT_SIZE, VERTEX_SHAPE, VERTEX_SIZE, VERTEX_TEXT_Z},
    observers::{
        on_vertex_clicked, on_vertex_dragged, on_vertex_dragging, on_vertex_drop,
        on_vertex_hovered, on_vertex_out,
    },
};

/// A bundle for spawning a vertex in the graph.
#[derive(Bundle)]
pub struct VertexBundle {
    vertex: Vertex,
    click_tracker: ClickTracker,
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
            font: TextFont::from_font_size(VERTEX_LABEL_FONT_SIZE),
            mesh: Mesh2d(meshes.add(VERTEX_SHAPE)),
            material: MeshMaterial2d(materials.add(VERTEX_COLOR)),
            position: Position(position),
        }
    }

    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        position: Vec2,
    ) -> Entity {
        let entity_id = commands.spawn(Self::new(meshes, materials, position)).id();

        commands.entity(entity_id)
            .observe(on_vertex_clicked)
            .observe(on_vertex_hovered)
            .observe(on_vertex_out)
            .observe(on_vertex_dragging)
            .observe(on_vertex_drop)
            .observe(on_vertex_dragged);

        commands.entity(entity_id).with_children(|parent| {
            parent.spawn((
                Text2d::new(""),
                TextFont::from_font_size(VERTEX_LABEL_FONT_SIZE),
                Transform::from_xyz(0.0, -VERTEX_SIZE - 15.0, VERTEX_TEXT_Z),
            ));
        });

        entity_id
    }
}
