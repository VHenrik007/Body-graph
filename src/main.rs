use bevy::{gizmos::circles::EllipseBuilder, prelude::*};

// CONSTANTS

const VERTEX_SIZE: f32 = 10.;
const VERTEX_SHAPE: Circle = Circle::new(VERTEX_SIZE);
const VERTEX_COLOR: Color = Color::srgb(1., 0., 0.);

const EDGE_COLOR: Color = Color::srgb(0., 1., 0.);
const TEMP_EDGE_COLOR: Color = Color::srgb(0., 0., 1.);

const BG_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);

// COMPONTENTS

#[derive(Component, Default, Clone)]
#[require(Transform)]
struct Position(Vec2);

#[derive(Component)]
#[require(Position)]
struct Vertex;

#[derive(Component)]
struct DirectedEdge {
    from: Entity,
    to: Entity,
}

#[derive(Component)]
struct TemporaryDirectedEdge {
    from: Option<Entity>,
    to: Vec2,
}

#[derive(Component)]
#[require(
    Transform = Transform {
        translation: Vec3 { x: 0., y: 0., z: -1. },
        ..Default::default()
    }
)]
struct Background;


// RESOURCES

#[derive(Resource)]
struct HoveredEntity(Option<Entity>);

// SYSTEMS

// Startup (spawning)

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>
) {
    let bg_mesh = meshes.add(Rectangle::from_size(window.size()));
    let bg_material = materials.add(BG_COLOR);

    commands.spawn((
        Background,
        Mesh2d(bg_mesh),
        MeshMaterial2d(bg_material)
    )).observe(on_bg_clicked);
}

fn spawn_graph(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let vertex_mesh = meshes.add(VERTEX_SHAPE);
    let vertex_material = materials.add(VERTEX_COLOR);
    let vertex1_pos = Position(Vec2::new(100., 100.));
    let vertex2_pos = Position(Vec2::new(-100., -100.));

    let vertex1 = commands.spawn((
        Vertex,
        Mesh2d(vertex_mesh.clone()),
        MeshMaterial2d(vertex_material.clone()),
        vertex1_pos.clone(),
    ))
    .observe(on_vertex_hovered)
    .observe(on_vertex_out)
    .observe(on_vertex_dragging)
    .observe(on_vertex_drop)
    .observe(on_vertex_dragged)
    .id();
    let vertex2 = commands.spawn((
        Vertex,
        Mesh2d(vertex_mesh),
        MeshMaterial2d(vertex_material),
        vertex2_pos.clone(),
    ))
    .observe(on_vertex_hovered)
    .observe(on_vertex_out)
    .observe(on_vertex_dragging)
    .observe(on_vertex_drop)
    .observe(on_vertex_dragged)
    .id();

    let edge_shape = Segment2d::new(Vec2::ZERO, Vec2::X);
    let edge_component = DirectedEdge {
        from: vertex1,
        to: vertex2
    };

    commands.spawn((
        edge_component,
        Mesh2d(meshes.add(edge_shape)),
        MeshMaterial2d(materials.add(EDGE_COLOR)),
    ));

    commands.spawn((
        TemporaryDirectedEdge{
            from: None,
            to: Vec2::ZERO
        },
        Mesh2d(meshes.add(edge_shape)),
        MeshMaterial2d(materials.add(TEMP_EDGE_COLOR)),
    ));
}

// Updates
fn project_positions(
    mut positionables: Query<(&mut Transform, &Position), Without<Background>>
) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}

fn update_edge_transforms(
    edges: Query<(&DirectedEdge, &mut Transform), Without<Vertex>>,
    positions: Query<&Position>,
) {
    for (edge, mut transform) in edges {
        let Ok(from_pos) = positions.get(edge.from) else { continue };
        let Ok(to_pos) = positions.get(edge.to) else { continue };

        let direction = to_pos.0 - from_pos.0;
        let length = direction.length();
        let angle = direction.y.atan2(direction.x);

        transform.translation = from_pos.0.extend(-0.5);
        transform.rotation = Quat::from_rotation_z(angle);
        transform.scale.x = length;
    }
}

fn update_temp_edge_transform(
    edge: Single<(&TemporaryDirectedEdge, &mut Transform)>,
    positions: Query<&Position>,
) {
    let (edge, mut transform) = edge.into_inner();
    let Some(from_vertex) = edge.from else {
        transform.scale.x = 0.0;
        return
     };

    let Ok(from_pos) = positions.get(from_vertex) else { return };

    let direction = edge.to - from_pos.0;
    let length = direction.length();
    let angle = direction.y.atan2(direction.x);

    transform.translation = from_pos.0.extend(-0.5);
    transform.rotation = Quat::from_rotation_z(angle);
    transform.scale.x = length;
}

// Events

fn on_bg_clicked(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if click.button == PointerButton::Primary {
        let Some(click_position) = click.hit.position else { return; };

        commands.spawn((
            Vertex,
            Mesh2d(meshes.add(VERTEX_SHAPE)),
            MeshMaterial2d(materials.add(VERTEX_COLOR)),
            Position(Vec2::new(click_position.x, click_position.y)),
        ))
        .observe(on_vertex_hovered)
        .observe(on_vertex_out)
        .observe(on_vertex_dragging)
        .observe(on_vertex_drop)
        .observe(on_vertex_dragged);
    } else {
        println!("BG clicked something else: {:?}", click.button);
    }
}

fn on_vertex_hovered(
    over: On<Pointer<Over>>,
    mut hovered_entity: ResMut<HoveredEntity>,
) {
    hovered_entity.0 = Some(over.entity);
}

fn on_vertex_out(
    _out: On<Pointer<Out>>,
    mut hovered_entity: ResMut<HoveredEntity>,
) {
    hovered_entity.0 = None;
}

fn on_vertex_dragged(
    drag: On<Pointer<DragStart>>,
    mut temp_edge: Single<&mut TemporaryDirectedEdge>
) {
    if drag.button == PointerButton::Secondary {
        let Some(click_position) = drag.hit.position else { return; };

        temp_edge.from = Some(drag.entity);
        temp_edge.to.x = click_position.x;
        temp_edge.to.y = click_position.y;
    } else {
        println!("Vertex dragging with something unknown: {:?}", drag.button);
    }
}

fn on_vertex_drop(
    drag: On<Pointer<DragEnd>>,
    mut commands: Commands,
    mut temp_edge: Single<&mut TemporaryDirectedEdge>,
    camera: Single<(&Camera, &GlobalTransform)>,
    hovered: Res<HoveredEntity>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if drag.button == PointerButton::Secondary {
        temp_edge.from = None;
        if let Some(hovered_vertex) = hovered.0 {
            let edge_shape = Segment2d::new(Vec2::ZERO, Vec2::X);
            let edge_component = DirectedEdge {
                from: drag.entity,
                to: hovered_vertex,
            };

            commands.spawn((
                edge_component,
                Mesh2d(meshes.add(edge_shape)),
                MeshMaterial2d(materials.add(EDGE_COLOR)),
            ));
        } else {
            let (camera, camera_transform) = camera.into_inner();
            let cursor_screen_pos = drag.pointer_location.position;

            let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_screen_pos) else {
                return
            };

            let new_vertex = commands.spawn((
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
            .id();

            let edge_shape = Segment2d::new(Vec2::ZERO, Vec2::X);
            let edge_component = DirectedEdge {
                from: drag.entity,
                to: new_vertex,
            };

            commands.spawn((
                edge_component,
                Mesh2d(meshes.add(edge_shape)),
                MeshMaterial2d(materials.add(EDGE_COLOR)),
            ));
        }
    } else {
        println!("Vertex dragging with something unknown: {:?}", drag.button);
    }
}

fn on_vertex_dragging(
    drag: On<Pointer<Drag>>,
    mut positions: Query<&mut Position>,
    mut temp_edge: Single<&mut TemporaryDirectedEdge>,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    if drag.button == PointerButton::Primary {
        let Ok(mut position) = positions.get_mut(drag.entity) else { return; };
        position.0.x += drag.delta.x;
        position.0.y -= drag.delta.y;
    } else if drag.button == PointerButton::Secondary {
        let (camera, camera_transform) = camera.into_inner();
        let cursor_screen_pos = drag.pointer_location.position;

        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_screen_pos) {
            temp_edge.to = world_pos;
        }
    } else {
        println!("Vertex dragging with something unknown: {:?}", drag.button);
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .insert_resource(HoveredEntity(None))
        .add_systems(Startup, (spawn_camera, spawn_background, spawn_graph))
        .add_systems(Update, (project_positions, update_edge_transforms, update_temp_edge_transform))
        .run();
}
