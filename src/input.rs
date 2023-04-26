use bevy::prelude::*;

use crate::MousePosition;

pub fn mouse_position_system(mut mouse_position: ResMut<MousePosition>, windows: Query<&Window>, cameras: Query<(&Camera, &GlobalTransform)>) {
    let window = windows.single();
    let (camera, camera_transform) = cameras.single();

    let position = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate());

    *mouse_position = MousePosition(position);
}
