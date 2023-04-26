use bevy::prelude::*;

mod a_star;
mod movement;
mod tilemap_generator;
mod ui;
mod attack;
mod select;
mod death;
mod turn;
mod input;

use input::mouse_position_system;
use movement::{display_path, snap, move_system};
use turn::turn_system;
use ui::{setup_ui, button_system, taskbar_visibility_system};
use select::select_system;
use attack::attack_system;
use death::death_system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(Turn::Player)
        .insert_resource(MousePosition(Some(Vec2::new(0.0, 0.0))))
        .add_startup_system(setup)
        .add_startup_system(setup_ui)
        .add_system(mouse_position_system)
        .add_system(taskbar_visibility_system)
        .add_system(select_system)
        .add_system(display_path)
        .add_system(button_system)
        .add_system(attack_system)
        .add_system(death_system)
        .add_system(turn_system)
        .add_system(move_system)
        .run();
}

const TILE_SIZE: f32 = 80.0;
const SPRITE_SIZE: f32 = 24.0;

#[derive(Component)]
pub struct ActionPoints {
    current: u8,
    max: u8
}

impl ActionPoints {
    fn new(max: u8) -> Self {
        ActionPoints { current: 0, max }
    }
}

#[derive(Component)]
pub enum Selected {
    Deciding,
    Movable,
    Moving(Vec<Vec2>),
    AbleToAttack,
    Attacking,
}

#[derive(Resource)]
pub enum Turn {
    Player,
    Enemy
}

#[derive(Resource)]
pub struct MousePosition(Option<Vec2>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default()); // kamera

    // spelare
    commands
        .spawn(CharacterBunble {
            sprite: SpriteBundle {
                texture: asset_server.load("sprites/heroes/Knight_Prototype.png"),
                transform: Transform::from_translation(Vec3::default())
                    .with_scale(Vec3::splat(TILE_SIZE / SPRITE_SIZE)),
                ..default()
            },
            ..default()
        });

    // fiende
    commands
        .spawn(EnemyBundle {
            sprite: SpriteBundle {
                texture: asset_server.load("sprites/enemies/Goblin.png"),
                transform: Transform::from_translation(Vec3::new(
                    snap(100., TILE_SIZE as u32, 1),
                    snap(40., TILE_SIZE as u32, 1),
                    0.,
                ))
                .with_scale(Vec3::splat(TILE_SIZE / SPRITE_SIZE)),
                ..default()
            },
            ..default()
        });
}

#[derive(Bundle)]
pub struct CharacterBunble {
    character: Character,
    health: Health,
    action_points: ActionPoints,
    sprite: SpriteBundle,
    movement_speed: MovementSpeed,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    health: Health,
    action_points: ActionPoints,
    sprite: SpriteBundle,
    movement_speed: MovementSpeed,
}

impl Default for CharacterBunble {
    fn default() -> Self {
        CharacterBunble {
            character: Character,
            health: Health(20),
            action_points: ActionPoints::new(2),
            sprite: SpriteBundle::default(),
            movement_speed: MovementSpeed(5),
        }
    }
}

impl Default for EnemyBundle {
    fn default() -> Self {
        EnemyBundle {
            enemy: Enemy,
            health: Health(20),
            action_points: ActionPoints::new(0),
            sprite: SpriteBundle::default(),
            movement_speed: MovementSpeed(2),
        }
    }
}

#[derive(Component)]
pub struct MovementSpeed(u8);

#[derive(Component)]
pub struct Character;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Marker;

#[derive(Component)]
pub struct Health(u32);

