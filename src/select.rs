use bevy::prelude::*;
use crate::a_star::a_star;
use crate::attack::{Defender, Attacker};
use crate::movement::{move_system, self, snap};
use crate::{TILE_SIZE, Character, Selected, Enemy, MousePosition, MovementSpeed, ActionPoints};

pub(crate) fn select_system(
    mouse_position: Res<MousePosition>,
    mut characters: Query<(&mut Transform, Entity, Option<&Selected>, &MovementSpeed, &mut ActionPoints), (With<Character>, Without<Enemy>)>,
    enemies: Query<(&Transform, Entity), (With<Enemy>, Without<Character>)>,
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
) {
    if let Some(mouse_position) = mouse_position.0 {
        for (transform, entity, selected, movement_speed, mut action_points) in characters.iter_mut() {
            if let Some(selected) = selected {
                if buttons.just_pressed(MouseButton::Left) {
                    match selected {
                        Selected::Movable => {
                            let path = a_star(Vec2::new(transform.translation.x, transform.translation.y), Vec2::new(snap(mouse_position.x, TILE_SIZE as u32, 1), snap(mouse_position.y, TILE_SIZE as u32, 1)));

                            let movement_cost = (path.len() as u8 - 2) / movement_speed.0 + 1;

                            if movement_cost <= (*action_points).current {
                                println!("cost: {:?}", movement_cost);
                                (*action_points).current -= movement_cost;
                                println!("points: {:?}", (*action_points).current);
                                commands.entity(entity).remove::<Selected>().insert(Selected::Moving(path));
                            }
                        },
                        Selected::AbleToAttack => {
                            for (enemy_transform, enemy_entity) in enemies.iter() {
                                if on_character(enemy_transform.translation, mouse_position) && action_points.current >= 1 {
                                    action_points.current -= 1;
                                    commands.entity(entity).remove::<Selected>().insert(Selected::Attacking).insert(Attacker);
                                    commands.entity(enemy_entity).insert(Defender);
                                }
                            }
                        },
                        Selected::Moving(_) | Selected::Attacking => {},
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
