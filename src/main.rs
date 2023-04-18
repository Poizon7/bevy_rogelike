use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

mod a_star;
mod movement;

use movement::{move_system, select_system};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(select_system)
        .add_system(move_system)
        .run();
}

const TILE_SIZE: f32 = 20.0;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::default().with_scale(Vec3::splat(TILE_SIZE)),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        })
        .insert(Character);

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::from_translation(Vec3::new(100., 0., 0.))
                .with_scale(Vec3::splat(TILE_SIZE)),
            material: materials.add(ColorMaterial::from(Color::RED)),
            ..default()
        })
        .insert(Enemy)
        .insert(Health(10));
}

#[derive(Component)]
struct Character;

#[derive(Component)]
struct Selected;

#[derive(Component)]
struct Marker;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Health(u32);

#[derive(Component)]
struct Active;

#[derive(Component)]
struct Attacker;

#[derive(Component)]
struct Defender;