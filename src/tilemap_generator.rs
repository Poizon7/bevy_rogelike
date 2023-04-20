use std::fs::read_to_string;

use crate::{SPRITE_SIZE, TILE_SIZE};
use bevy::{prelude::*, render::view::Layer, sprite::MaterialMesh2dBundle};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct TileMap {
    compressionlevel: i32,
    height: u32,
    infinite: bool,
    layers: Vec<Layers>,
    nextlayerid: u32,
    nextobjectid: u32,
    orientation: String,
    renderorder: String,
    tiledversion: String,
    tileheight: u32,
    tilesets: Vec<TileSets>,
    tilewidth: u32,
    version: String,
    width: u32,
}

#[derive(Deserialize, Debug)]
struct TileSets {
    firstgid: u32,
    source: String,
}

#[derive(Deserialize, Debug)]
struct Layers {
    data: Vec<i32>,
    height: u32,
    id: u32,
    name: String,
    opacity: u32,
    visible: bool,
    width: u32,
    x: i32,
    y: i32,
}

pub(crate) fn generate_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    //let data_str = read_to_string(String::from("tilemaps/basic_tilemap.json")).unwrap();

    let data_str = r#"{ "compressionlevel":-1,
    "height":10,
    "infinite":false,
    "layers":[
           {
            "data":[1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
               1, 2, 2, 2, 2, 2, 2, 2, 2, 1,
               1, 2, 2, 2, 2, 2, 2, 2, 2, 1,
               1, 2, 2, 1, 1, 2, 2, 2, 2, 1,
               1, 2, 2, 1, 1, 2, 2, 2, 2, 1,
               1, 2, 2, 2, 2, 2, 2, 2, 2, 1,
               1, 2, 2, 2, 2, 2, 2, 2, 2, 1,
               1, 2, 2, 2, 2, 2, 1, 1, 2, 1,
               1, 2, 2, 2, 2, 2, 2, 2, 2, 1,
               1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            "height":10,
            "id":1,
            "name":"Tile Layer 1",
            "opacity":1,
            "visible":true,
            "width":10,
            "x":0,
            "y":0
           }],
    "nextlayerid":3,
    "nextobjectid":1,
    "orientation":"orthogonal",
    "renderorder":"right-down",
    "tiledversion":"1.10.1",
    "tileheight":24,
    "tilesets":[
           {
            "firstgid":1,
            "source":"basic.tsx"
           }],
    "tilewidth":24,
    "version":"1.10",
    "width":10
   }"#;

    let data: TileMap = serde_json::from_str(&data_str).unwrap();

    //println!("{:?}", data);
}
