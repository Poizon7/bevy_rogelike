use bevy::prelude::*;

use crate::{ActionPoints, Turn, Character, Enemy, Selected};

pub fn turn_system(mut characters: Query<&mut ActionPoints, (With<Character>, Without<Enemy>)>, mut enemies: Query<(&mut ActionPoints, Entity), (With<Enemy>, Without<Character>)>, selected: Query<&Selected>, mut commands: Commands, mut turn: ResMut<Turn>) {
    match *turn {
        Turn::Player => {
            if characters.iter().all(|x| x.current == 0) && selected.is_empty() {
                *turn = Turn::Enemy;

                for (mut enemy, entity) in enemies.iter_mut() {
                    enemy.current = enemy.max;
                    commands.entity(entity).insert(Selected::Deciding);
                }
            }
        }
        Turn::Enemy => {
            if enemies.iter().all(|(x, _)| x.current == 0) && selected.is_empty() {
                *turn = Turn::Player;

                for mut character in characters.iter_mut() {
                    character.current = character.max;
                }
            }
        }
    }
}
