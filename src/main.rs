use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

mod a_star;
mod movement;

use movement::{move_system, select_system, snap};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_startup_system(setup)
        .add_system(select_system)
        .add_system(move_system)
        .run();
}

const TILE_SIZE: f32 = 80.0;
const SPRITE_SIZE: f32 = 24.0;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default()); // kamera

    // spelare
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("sprites/heroes/Knight_Prototype.png"),
            transform: Transform::from_translation(Vec3::default())
                .with_scale(Vec3::splat(TILE_SIZE / SPRITE_SIZE)),
            ..default()
        })
        .insert(Character);

    // fiende
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("sprites/enemies/Goblin.png"),
            transform: Transform::from_translation(Vec3::new(
                snap(100., TILE_SIZE as u32, 1),
                snap(40., TILE_SIZE as u32, 1),
                0.,
            ))
            .with_scale(Vec3::splat(TILE_SIZE / SPRITE_SIZE)),
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
