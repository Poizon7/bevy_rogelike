use bevy::prelude::*;

use crate::Health;

pub fn death_system(dead: Query<(&Health, Entity)>, mut commands: Commands) {
    for (health, entity) in dead.iter() {
        if health.0 <= 0 {
            commands.entity(entity).despawn();
            println!("dead");
        }
    }
}
