use bevy::{picking::window, prelude::*};

// CONSTANTS

const VERTEX_SIZE: f32 = 10.;
const VERTEX_SHAPE: Circle = Circle::new(VERTEX_SIZE);
const VERTEX_COLOR: Color = Color::srgb(1., 0., 0.);

const EDGE_COLOR: Color = Color::srgb(0., 1., 0.);

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
#[require(
    Transform = Transform {
        translation: Vec3 { x: 0., y: 0., z: -1. },
        ..Default::default()
    }
)]
struct Background;


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
    )).observe(on_vertex_clicked).id();
    let vertex2 = commands.spawn((
        Vertex,
        Mesh2d(vertex_mesh),
        MeshMaterial2d(vertex_material),
        vertex2_pos.clone(),
    )).observe(on_vertex_clicked).id();

    let edge_shape = Segment2d::new(vertex1_pos.0, vertex2_pos.0);
    let edge_mesh = meshes.add(edge_shape);
    let edge_material = materials.add(EDGE_COLOR);
    let edge_component = DirectedEdge {
        from: vertex1,
        to: vertex2
    };

    let _edge = commands.spawn((
        edge_component,
        Mesh2d(edge_mesh),
        MeshMaterial2d(edge_material),
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

// Events

fn on_bg_clicked(_click: On<Pointer<Click>>,) {
    println!("BG clicked");
}

fn on_vertex_clicked(_click: On<Pointer<Click>>,) {
    println!("Vertex clicked");
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_systems(Startup, (spawn_camera, spawn_background, spawn_graph))
        .add_systems(Update, project_positions)
        .run();
}
