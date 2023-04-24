use bevy::prelude::*;

mod a_star;
mod movement;
mod tilemap_generator;
mod ui;
mod attack;
mod select;
mod death;

use movement::{move_system, snap};
use ui::{setup_ui, button_system, taskbar_visibility_system};
use select::select_system;
use attack::attack_system;
use death::death_system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_startup_system(setup)
        .add_system(select_system)
        .add_system(move_system)
        .add_startup_system(setup_ui)
        .add_system(taskbar_visibility_system)
        .add_system(button_system)
        .add_system(attack_system)
        .add_system(death_system)
        .run();
}

const TILE_SIZE: f32 = 80.0;
const SPRITE_SIZE: f32 = 24.0;

#[derive(Component)]
pub enum Selected {
    Deciding,
    Movable,
    Moving,
    AbleToAttack,
    Attacking,
}

fn setup(
    mut commands: Commands,
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
struct Marker;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
pub struct Health(u32);

