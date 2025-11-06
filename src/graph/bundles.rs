use bevy::prelude::*;

use crate::graph::{
    components::{ClickTracker, Position, Vertex},
    constants::{VERTEX_COLOR, VERTEX_LABEL_FONT_SIZE, VERTEX_SHAPE, VERTEX_SIZE},
    observers::{
        on_vertex_clicked, on_vertex_dragged, on_vertex_dragging, on_vertex_drop,
        on_vertex_hovered, on_vertex_out,
    },
};

#[derive(Bundle)]
pub struct VertexBundle {
    vertex: Vertex,
    click_tracker: ClickTracker,
    // If not commented, the first vertex behaves oddly
    // text: Text2d,
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
            // If not commented, the first vertex behaves oddly
            // text: Text2d::default(),
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
        label: Option<&str>,
    ) -> Entity {
        let entity_id = commands.spawn(Self::new(meshes, materials, position)).id();

        commands.entity(entity_id)
            .observe(on_vertex_clicked)
            .observe(on_vertex_hovered)
            .observe(on_vertex_out)
            .observe(on_vertex_dragging)
            .observe(on_vertex_drop)
            .observe(on_vertex_dragged);

        if let Some(label_text) = label {
            commands.entity(entity_id).with_children(|parent| {
                parent.spawn((
                    Text2d::new(label_text),
                    TextFont::from_font_size(VERTEX_LABEL_FONT_SIZE),
                    Transform::from_xyz(0.0, VERTEX_SIZE + 5.0, 1.0),
                ));
            });
        }

        entity_id
    }
}
