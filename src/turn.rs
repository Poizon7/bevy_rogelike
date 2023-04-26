use bevy::prelude::*;

use crate::{ActionPoints, Turn, Character, Enemy};

    pub fn turn_system(mut characters: Query<&mut ActionPoints, (With<Character>, Without<Enemy>)>, mut enemies: Query<&mut ActionPoints, (With<Enemy>, Without<Character>)>, mut turn: ResMut<Turn>) {
    match *turn {
        Turn::Player => {
            if characters.iter().all(|x| x.current == 0) {
                println!("enemy turn");
                *turn = Turn::Enemy;

                for mut enemy in enemies.iter_mut() {
                    enemy.current = enemy.max;
                }
            }
        }
        Turn::Enemy => {
            if enemies.iter().all(|x| x.current == 0) {
                println!("player turn");
                *turn = Turn::Player;

                for mut character in characters.iter_mut() {
                    character.current = character.max;
                }
            }
        }
    }
}
