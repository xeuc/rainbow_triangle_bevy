use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Make the scene visible
    commands.spawn(Camera2dBundle::default());

    // Define the triangle shape
    let mut mesh = Mesh::from(Triangle2d::default());

    // Define the colors of the corners
    let vertex_colors: Vec<[f32; 4]> = vec![
        Color::RED.as_rgba_f32(),
        Color::GREEN.as_rgba_f32(),
        Color::BLUE.as_rgba_f32(),
    ];
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);
    let mesh_handle: Mesh2dHandle = meshes.add(mesh).into();
    
    // Render the triangle
    commands.spawn(MaterialMesh2dBundle {
        mesh: mesh_handle,
        transform: Transform::from_scale(Vec3::splat(512.)),
        material: materials.add(ColorMaterial::default()),
        ..default()
    });
}

