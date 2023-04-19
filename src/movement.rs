use crate::{Character, Marker, Selected, TILE_SIZE};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub(crate) fn select_system(
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    characters: Query<(&Transform, Entity), (With<Character>, Without<Selected>)>,
    selected: Query<Entity, With<Selected>>,
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = windows.single();
    let (camera, camera_transform) = cameras.single();

    let mut hover: Option<Entity> = None;

    if let Some(mouse_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        for (transform, entity) in characters.iter() {
            if transform.translation.x - TILE_SIZE / 2.0 < mouse_position.x
                && transform.translation.x + TILE_SIZE / 2.0 > mouse_position.x
                && transform.translation.y - TILE_SIZE / 2.0 < mouse_position.y
                && transform.translation.y + TILE_SIZE / 2.0 > mouse_position.y
            {
                hover = Some(entity);
                commands
                    .entity(entity)
                    .remove::<Handle<ColorMaterial>>()
                    .insert(materials.add(ColorMaterial::from(Color::GREEN)));
            } else {
                commands
                    .entity(entity)
                    .remove::<Handle<ColorMaterial>>()
                    .insert(materials.add(ColorMaterial::from(Color::PURPLE)));
            }
        }
    }

    if buttons.just_pressed(MouseButton::Left) {
        if let Some(entity) = hover {
            commands.entity(entity).insert(Selected);
            commands
                .entity(entity)
                .remove::<Handle<ColorMaterial>>()
                .insert(materials.add(ColorMaterial::from(Color::BLUE)));
        } else {
            for entity in selected.iter() {
                commands.entity(entity).remove::<Selected>();
                commands
                    .entity(entity)
                    .remove::<Handle<ColorMaterial>>()
                    .insert(materials.add(ColorMaterial::from(Color::PURPLE)));
            }
        }
    }
}

pub(crate) fn move_system(
    mut selected: Query<&mut Transform, With<Selected>>,
    mut markers: Query<(&mut Transform, Entity), (With<Marker>, Without<Selected>)>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<Input<MouseButton>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = windows.single();
    let (camera, camera_transform) = cameras.single();

    if let Some(mut transform) = selected.iter_mut().next() {
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
                if buttons.just_pressed(MouseButton::Left) {
                    transform.translation = marker.0.translation;
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
    }
}

pub(crate) fn snap(original: f32, numinator: u32, denuminator: u32) -> f32 {
    (original / numinator as f32 * denuminator as f32).round() * numinator as f32
        / denuminator as f32
}
