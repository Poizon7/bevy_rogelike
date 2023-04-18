use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(select)
        .run();
}

const TILE_SIZE: f32 = 20.0;

#[derive(Component)]
struct Character;

#[derive(Component)]
struct Selected;

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(TILE_SIZE)),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..default()
    }).insert(Character);
}

fn select(windows: Query<&Window>, cameras: Query<(&Camera, &GlobalTransform)>, characters: Query<(&Transform, Entity), (With<Character>, Without<Selected>)>, selected: Query<Entity, With<Selected>>, mut commands: Commands, buttons: Res<Input<MouseButton>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let window = windows.single();
    let (camera, camera_transform) = cameras.single();

    let mut hover: Option<Entity> = None;

    if let Some(mouse_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate()) {
        for (transform, entity) in characters.iter() {
            if transform.translation.x - TILE_SIZE / 2.0 < mouse_position.x && transform.translation.x + TILE_SIZE / 2.0 > mouse_position.x && transform.translation.y - TILE_SIZE / 2.0 < mouse_position.y && transform.translation.y + TILE_SIZE / 2.0 > mouse_position.y {
                hover = Some(entity);
                commands.entity(entity).remove::<Handle<ColorMaterial>>().insert(materials.add(ColorMaterial::from(Color::GREEN)));
            }
            else {
                commands.entity(entity).remove::<Handle<ColorMaterial>>().insert(materials.add(ColorMaterial::from(Color::PURPLE)));
            }
        }
    }

    if buttons.just_pressed(MouseButton::Left) {
        if let Some(entity) = hover {
            commands.entity(entity).insert(Selected);
            commands.entity(entity).remove::<Handle<ColorMaterial>>().insert(materials.add(ColorMaterial::from(Color::BLUE)));
        }
        else {
            for entity in selected.iter() {
                commands.entity(entity).remove::<Selected>();
                commands.entity(entity).remove::<Handle<ColorMaterial>>().insert(materials.add(ColorMaterial::from(Color::PURPLE)));
            }
        }
    }
}
