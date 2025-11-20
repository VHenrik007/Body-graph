use bevy::prelude::*;

use crate::graph::{
    components::{ClickTracker, DirectedEdge, Position, Vertex},
    constants::{
        EDGE_COLOR, EDGE_SHAPE, VERTEX_COLOR, VERTEX_LABEL_FONT_SIZE, VERTEX_SHAPE, VERTEX_SIZE,
        VERTEX_TEXT_Z,
    },
    picking_observers::{
        on_edge_clicked, on_edge_hovered, on_edge_out, on_vertex_clicked, on_vertex_dragged,
        on_vertex_dragging, on_vertex_drop, on_vertex_hovered, on_vertex_out,
    },
};

/// A bundle for spawning a vertex in the graph.
#[derive(Bundle, Debug)]
pub struct VertexBundle {
    vertex: Vertex,
    click_tracker: ClickTracker,
    font: TextFont,
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    position: Position,
}

impl VertexBundle {
    /// Vertices are spawned at a given location. Meshes and materials are fix.
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

    pub fn new_with_label(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        position: Vec2,
        label: &str,
    ) -> Self {
        Self {
            vertex: Vertex {
                label: label.to_string(),
            },
            click_tracker: ClickTracker::default(),
            font: TextFont::from_font_size(VERTEX_LABEL_FONT_SIZE),
            mesh: Mesh2d(meshes.add(VERTEX_SHAPE)),
            material: MeshMaterial2d(materials.add(VERTEX_COLOR)),
            position: Position(position),
        }
    }

    /// Extends the spawning function with the observers that are true for all
    /// vertices and adds the Text2d children.
    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        position: Vec2,
    ) -> Entity {
        let entity_id = commands.spawn(Self::new(meshes, materials, position)).id();

        commands
            .entity(entity_id)
            .observe(on_vertex_clicked)
            .observe(on_vertex_hovered)
            .observe(on_vertex_out)
            .observe(on_vertex_dragging)
            .observe(on_vertex_drop)
            .observe(on_vertex_dragged);

        VertexBundle::add_children(commands, entity_id);

        entity_id
    }

    pub fn add_children(commands: &mut Commands, entity_id: Entity) {
        commands.entity(entity_id).with_children(|parent| {
            parent.spawn((
                Text2d::new(""),
                TextFont::from_font_size(VERTEX_LABEL_FONT_SIZE),
                Transform::from_xyz(0.0, -VERTEX_SIZE - 15.0, VERTEX_TEXT_Z),
            ));
        });
    }

    pub fn add_children_with_label(commands: &mut Commands, entity_id: Entity, label: &str) {
        commands.entity(entity_id).with_children(|parent| {
            parent.spawn((
                Text2d::new(label),
                TextFont::from_font_size(VERTEX_LABEL_FONT_SIZE),
                Transform::from_xyz(0.0, -VERTEX_SIZE - 15.0, VERTEX_TEXT_Z),
            ));
        });
    }
}

/// A bundle for spawning edges
#[derive(Bundle)]
pub struct DirectedEdgeBundle {
    directed_edge: DirectedEdge,
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
}

impl DirectedEdgeBundle {
    /// Edges are defined by their from and to vertices.
    pub fn new(
        from: Entity,
        to: Entity,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> Self {
        Self {
            directed_edge: DirectedEdge { from, to },
            mesh: Mesh2d(meshes.add(EDGE_SHAPE)),
            material: MeshMaterial2d(materials.add(EDGE_COLOR)),
        }
    }

    /// Extends the spawning function with the observers that are true for all
    /// vertices and adds the Text2d children.
    pub fn spawn(
        from: Entity,
        to: Entity,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> Entity {
        let entity_id = commands.spawn(Self::new(from, to, meshes, materials)).id();

        commands
            .entity(entity_id)
            .observe(on_edge_clicked)
            .observe(on_edge_hovered)
            .observe(on_edge_out);

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
