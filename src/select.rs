use bevy::prelude::*;
use crate::attack::{Defender, Attacker};
use crate::{TILE_SIZE, Character, Selected, Enemy};

pub(crate) fn select_system(
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    characters: Query<(&Transform, Entity, Option<&Selected>), With<Character>>,
    enemies: Query<(&Transform, Entity), With<Enemy>>,
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
) {
    let window = windows.single();
    let (camera, camera_transform) = cameras.single();

    if let Some(mouse_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        
        for (transform, entity, selected) in characters.iter() {
            if let Some(selected) = selected {
                if buttons.just_pressed(MouseButton::Left) {
                    match selected {
                        Selected::Movable => {
                        },
                        Selected::AbleToAttack => {
                            for (enemy_transform, enemy_entity) in enemies.iter() {
                                if on_character(enemy_transform.translation, mouse_position) {
                                    commands.entity(entity).insert(Attacker);
                                    commands.entity(enemy_entity).insert(Defender);
                                }
                            }
                        },
                        Selected::Moving | Selected::Attacking => {},
                        Selected::Deciding => {
                            if !on_character(transform.translation, mouse_position) {
                                commands.entity(entity).remove::<Selected>();
                            }
                        }
                    }
                }
                else if buttons.just_pressed(MouseButton::Right) {
                    match selected {
                        Selected::Movable | Selected::AbleToAttack => {
                            commands.entity(entity).remove::<Selected>().insert(Selected::Deciding);
                        },
                        _ => {}
                    }
                }
            }
            else {
                if buttons.just_pressed(MouseButton::Left) && on_character(transform.translation, mouse_position) {
                    commands.entity(entity).insert(Selected::Deciding);
                }
            }
        }
    }
}

fn on_character(character_position: Vec3, mouse_position: Vec2) -> bool {
    character_position.x - TILE_SIZE / 2.0 < mouse_position.x &&
    character_position.x + TILE_SIZE / 2.0 > mouse_position.x &&
    character_position.y - TILE_SIZE / 2.0 < mouse_position.y &&
    character_position.y + TILE_SIZE / 2.0 > mouse_position.y
}
