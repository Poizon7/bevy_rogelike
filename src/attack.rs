use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{a_star::a_star, Health, Selected, TILE_SIZE};

#[derive(Component)]
pub struct Attacker;

#[derive(Component)]
pub struct Defender;

pub fn attack_system(
    attackers: Query<(&Transform, Entity), With<Attacker>>,
    mut defenders: Query<(&Transform, &mut Health, Entity), With<Defender>>,
    mut commands: Commands,
) {
    for (attacker, attacker_entity) in attackers.iter() {
        for (defender_transform, mut health, defender_entity) in defenders.iter_mut() {
            let distance = a_star(
                Vec2::new(attacker.translation.x, attacker.translation.y),
                Vec2::new(
                    defender_transform.translation.x,
                    defender_transform.translation.y,
                ),
            );

            if distance.len() == 2 {
                health.0 = match health.0.checked_sub(5) {
                    Some(health) => health,
                    None => 0,
                };
                commands.entity(attacker_entity).remove::<Attacker>();
                commands.entity(defender_entity).remove::<Defender>();
            }
        }
    }
}

// Visar range för attacken hoss atacker
pub fn attack_range(
    mut attackers: Query<(&Transform, Entity, Option<&Selected>)>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let range: u32 = 1; // ändra till att ta in dynamiskt
    for (attacker, attacker_entity, selected) in attackers.iter_mut() {
        if let Some(selected) = selected {
            match selected {
                Selected::AbleToAttack => commands
                    .entity(attacker_entity)
                    .insert(AttackPreview(false)),
                _ => continue,
            };
        }
    }
}

#[derive(Component)]
pub struct AttackPreview(bool);

/*commands.spawn(MaterialMesh2dBundle {
    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
    material: materials.add(ColorMaterial::from(Color::Rgba {
        red: (255.),
        green: (0.),
        blue: (0.),
        alpha: (0.1),
    })),
    transform: Transform::from_translation(transform.translation)
        .with_scale(Vec3::splat(TILE_SIZE * (1 + 2) as f32)), // byt ut 1 mot range
    ..default()
}); */
