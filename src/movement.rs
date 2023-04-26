use crate::{Marker, Selected, TILE_SIZE, a_star::a_star, MousePosition};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
pub struct Path;


pub(crate) fn move_system(mut actors: Query<(&mut Transform, &mut Selected, Entity)>, mut commands: Commands) {
    for (mut transform, mut selected, entity) in actors.iter_mut() {
        match selected.as_mut() {
            Selected::Moving(ref mut path) => {
                let position = path.pop().unwrap();
                transform.translation = Vec3::new(position.x, position.y, 0.0);
                if path.len() == 0 {
                    commands.entity(entity).remove::<Selected>().insert(Selected::Deciding);
                }
            },
            _ => {}
        }

    }
}

pub fn display_path(
    selected: Query<(&Transform, &Selected)>,
    path: Query<Entity, With<Path>>,
    mouse_position: Res<MousePosition>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
    if let Some((transform, selected)) = selected.iter().next() {
        if matches!(selected, Selected::Movable) {
            if let Some(mouse_position) = mouse_position.0 {
                let end_position = Vec2::from((
                    snap(mouse_position.x, TILE_SIZE as u32, 1),
                    snap(mouse_position.y, TILE_SIZE as u32, 1),
                ));

                for tile in path.iter() {
                    commands.entity(tile).despawn();
                }

                let path = a_star(Vec2::new(transform.translation.x, transform.translation.y), end_position);

                for (i, node) in path.iter().enumerate() {
                    let color = if path.len() as isize - i as isize - 2 < 5 {
                        materials.add(ColorMaterial::from(Color::GREEN))
                    } else if path.len() as isize - i as isize - 2 < 10 {
                        materials.add(ColorMaterial::from(Color::ORANGE))
                    } else {
                        materials.add(ColorMaterial::from(Color::RED))
                    };
                    commands.spawn(MaterialMesh2dBundle {
                        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                        transform: Transform::from_xyz(node.x, node.y, 0.0).with_scale(Vec3::splat(TILE_SIZE / 2.0)),
                        material: color,
                        ..default()
                    }).insert(Path);
                }
            }
        } else {
            for tile in path.iter() {
                commands.entity(tile).despawn();
            }
        }
    }
}

pub(crate) fn snap(original: f32, numinator: u32, denuminator: u32) -> f32 {
    (original / numinator as f32 * denuminator as f32).round() * numinator as f32
        / denuminator as f32
}
