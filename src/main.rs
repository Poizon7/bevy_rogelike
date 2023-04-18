use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

mod movement;
mod a_star;

use movement::{select_system, move_system};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(select_system)
        .add_system(move_system)
        .run();
}

const TILE_SIZE: f32 = 20.0;

#[derive(Component)]
struct Character;

#[derive(Component)]
struct Selected;

#[derive(Component)]
struct Marker;

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(TILE_SIZE)),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..default()
    }).insert(Character);
}
