use bevy::prelude::*;

use crate::{ActionPoints, Enemy, Character, a_star::a_star, MovementSpeed, Selected, attack::{Attacker, Defender}, Turn};

pub fn ai(turn: Res<Turn>, mut enemies: Query<(&Transform, &mut ActionPoints, &MovementSpeed, &Selected, Entity), (With<Enemy>, Without<Character>)>, characters: Query<(&Transform, Entity), (With<Character>, Without<Enemy>)>, mut commands: Commands) {
    if matches!(*turn, Turn::Enemy) {
        for (enemy_transform, mut action_points, movement_speed, selected, entity) in enemies.iter_mut() {
            match *selected {
                Selected::Deciding => {
                    let mut path = characters.iter()
                        .map(|character| a_star(Vec2::new(enemy_transform.translation.x, enemy_transform.translation.y), Vec2::new(character.0.translation.x, character.0.translation.y)))
                        .min_by_key(|x| x.len())
                        .unwrap();

                    let movement_cost = path.len() as u8 / movement_speed.0 + 1;

                    if path.len() == 2  && (*action_points).current >= 1 {
                        let defender = characters.iter().find(|x| Vec2::new(x.0.translation.x, x.0.translation.y) == *path.first().unwrap()).unwrap().1;
                        
                        commands.entity(entity).insert(Attacker);
                        commands.entity(defender).insert(Defender);

                        (*action_points).current -= 1;
                    } else if movement_cost <= (*action_points).current {
                        path.remove(0);
                        commands.entity(entity).insert(Selected::Moving(path));
                        (*action_points).current -= movement_cost;
                    } else {
                        action_points.current = 0;
                        commands.entity(entity).remove::<Selected>();
                    }
                },
                _ => {}
            }
        }
    }
}
