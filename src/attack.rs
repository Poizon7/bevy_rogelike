use bevy::prelude::*;

use crate::{a_star::a_star, Health, Selected};


#[derive(Component)]
pub struct Attacker;

#[derive(Component)]
pub struct Defender;

pub fn attack_system(attackers: Query<(&Transform, Entity), With<Attacker>>, mut defenders: Query<(&Transform, &mut Health, Entity), With<Defender>>, mut commands: Commands) {
    for (attacker, attacker_entity) in attackers.iter() {
        for (defender_transform, mut health, defender_entity) in defenders.iter_mut() {
            let distance = a_star(Vec2::new(attacker.translation.x, attacker.translation.y), Vec2::new(defender_transform.translation.x, defender_transform.translation.y));

            if distance.len() == 2 {
                health.0 = match health.0.checked_sub(5) {
                    Some(health) => health,
                    None => 0
                };
                commands.entity(attacker_entity).remove::<Attacker>();
                commands.entity(defender_entity).remove::<Defender>();
            }
        }
    }
}
