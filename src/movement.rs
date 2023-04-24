use crate::{Marker, Selected, TILE_SIZE, a_star::a_star};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
pub struct Path;


pub(crate) fn move_system(
    mut selected: Query<(&mut Transform, &Selected, Entity)>,
    mut markers: Query<(&mut Transform, Entity), (With<Marker>, Without<Selected>)>,
    path: Query<Entity, With<Path>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<Input<MouseButton>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = windows.single();
    let (camera, camera_transform) = cameras.single();

    if let Some((mut transform, selected, entity)) = selected.iter_mut().next() {
        if matches!(selected, Selected::Movable) {
            if let Some(mut marker) = markers.iter_mut().next() {
                if let Some(mouse_position) = window
                    .cursor_position()
                    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                    .map(|ray| ray.origin.truncate())
                {
                    marker.0.translation = Vec3::from((
                        snap(mouse_position.x, TILE_SIZE as u32, 1),
                        snap(mouse_position.y, TILE_SIZE as u32, 1),
                        0.0,
                    ));

                    for tile in path.iter() {
                        commands.entity(tile).despawn();
                    }

                    let path = a_star(Vec2::new(transform.translation.x, transform.translation.y), Vec2::new(marker.0.translation.x, marker.0.translation.y));
                    for node in path {
                        commands.spawn(MaterialMesh2dBundle {
                            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                            transform: Transform::from_xyz(node.x, node.y, 0.0).with_scale(Vec3::splat(TILE_SIZE / 2.0)),
                            material: materials.add(ColorMaterial::from(Color::PURPLE)),
                            ..default()
                        }).insert(Path);
                    }

                    if buttons.just_pressed(MouseButton::Left) {
                        transform.translation = marker.0.translation;
                        commands.entity(entity).remove::<Selected>();
                        commands.entity(entity).insert(Selected::Deciding);
                    }
                }
            } else {
                commands
                    .spawn(MaterialMesh2dBundle {
                        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                        transform: Transform::default().with_scale(Vec3::splat(TILE_SIZE)),
                        material: materials.add(ColorMaterial::from(Color::PURPLE)),
                        ..default()
                    })
                    .insert(Marker);
            }
        } else {
            for (_, marker) in markers.iter() {
                commands.entity(marker).despawn();
            }
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
