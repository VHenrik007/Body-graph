use bevy::{prelude::*, window::CursorIcon};


/// Fires when a vertex rename is successful.
#[derive(Event)]
pub struct VertexRenamedEvent {
    /// The entity on which the rename should happen.
    pub entity: Entity,
    /// The new label for the entity.
    pub new_label: String,
}

/// Canvas click currently just spawns a vertex
/// on left click, might need screen_position
/// later on when adding context windows, and
/// extras for panning.
#[derive(Event)]
pub struct CanvasClickedEvent {
    /// Which button was used for the canvas.
    pub button: PointerButton,
    /// where on the canvas it happened.
    pub world_position: Vec2,
}

/// Clicking a vertex in any way.
/// Button currently does not matter as
/// it is differentiated from dragging where
/// it matters.
// Right click, idk what it should do
#[derive(Event)]
pub struct VertexClickedEvent {
    /// Vertex entity.
    pub entity: Entity,
    /// Where on the canvas.
    pub world_position: Vec2,
}

/// The end of the dragging event on
/// a vertex.
#[derive(Event)]
pub struct VertexDragDroppedEvent {
    /// Which vertex entity.
    pub entity: Entity,
    /// Which button started the drag.
    pub button: PointerButton,
    /// Where did the drag end on the canvas.
    pub world_position: Vec2,
}

/// Vertex is currently in draggin state and moved.
#[derive(Event)]
pub struct VertexDraggingEvent {
    /// Which vertex entity.
    pub entity: Entity,
    /// Which button started the drag.
    pub button: PointerButton,
    /// Where did the drag move on the canvas.
    pub world_position: Vec2,
}

/// Clickin on an edge.
#[derive(Event)]
pub struct EdgeClickedEvent {
    /// Which edge was clicked.
    pub entity: Entity,
    /// What button was used for the click.
    pub button: PointerButton,
    /// Where on the canvas did the click happpen.
    pub world_position: Vec2,
}

/// Cursor icon might change in various ways,
/// but since it inserts a component, I opted out
/// from making it into a resource.
#[derive(Event)]
pub struct UpdateCursorIconEvent {
    /// The cursor icon to insert as a component
    /// into the window.
    pub new_icon: CursorIcon
}

/// As a sub-case of clicking on an edge,
/// a vertex might be placed on it.
#[derive(Event)]
pub struct InsertVertexOnEdgeEvent {
    /// Which edge should be split.
    pub clicked_edge: Entity,
    /// Where should the new vertex be put.
    pub world_position: Vec2
}