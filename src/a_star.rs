use bevy::prelude::*;

struct Node {
    position: Vec2,
    f: f32,
    g: f32,
    h: f32
}

fn a_star(start_position: Vec2, end_position: Vec2) {
    // let open_set = start;
    // let cane_from;
}

fn manhattan_distance(start: Vec2, goal: Vec2) -> f32 {
    (start.x - goal.x).abs() + (start.y - goal.y).abs()
}
