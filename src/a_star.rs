use bevy::prelude::*;
use std::rc::Rc;

use crate::TILE_SIZE;

#[derive(Clone)]
struct Node {
    position: Vec2,
    parent: Option<Rc<Node>>,
    f: f32,
    g: f32,
    h: f32
}

pub (crate) fn a_star(start_position: Vec2, end_position: Vec2) -> Vec<Vec2> {
    let h = start_position.distance(end_position);
    let g = 0.0;
    let f = g + h;

    let start_node = Node {
        position: start_position,
        parent: None,
        f,
        g,
        h
    };
    let mut open_set = vec![start_node];

    loop {
        let reference = least(&open_set);
        let index = open_set.iter().position(|x| x.position == reference.position).unwrap();
        let current = open_set.remove(index);
        if current.position == end_position {
            return reconstruct_path(current);
        }

        let neighbors = vec![Vec2::new(current.position.x, current.position.y + TILE_SIZE),
        Vec2::new(current.position.x + TILE_SIZE, current.position.y),
        Vec2::new(current.position.x, current.position.y - TILE_SIZE),
        Vec2::new(current.position.x - TILE_SIZE, current.position.y)];

        for neighbor in neighbors {
            let g = current.g + current.position.distance(neighbor);
            if let Some(node) = open_set.iter_mut().find(|node| node.position == neighbor) {
                if g > node.g {
                    node.g = g;
                    node.f = node.h + node.g;
                    node.parent = Some(Rc::new(current.clone()));
                }
            }
            else {
                let h = neighbor.distance(end_position);
                let f = g + h;
                open_set.push(Node {
                    position: neighbor,
                    parent: Some(Rc::new(current.clone())),
                    f,
                    g,
                    h
                })
            }
        }
    }
}

fn reconstruct_path(node: Node) -> Vec<Vec2> {
    let mut total_path = Vec::new();
    let mut current = Some(Rc::new(node));

    while let Some(node) = current {
        total_path.push(node.position);

        if let Some(node) = node.parent.clone() {
            current = Some(node);
        }
        else {
            current = None;
        }
    }

    total_path
}

fn manhattan_distance(start: &Vec2, goal: &Vec2) -> f32 {
    (start.x - goal.x).abs() + (start.y - goal.y).abs()
}

fn least(node_list: &Vec<Node>) -> &Node {
    let mut least = node_list.first().unwrap();

    for node in node_list {
        if node.f < least.f {
            least = node;
        }
    }

    least
}
