use poem_openapi::{Object, OpenApi, param::Path, payload::Json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracer_macros::CRUD;

#[derive(CRUD, Debug, Serialize, Deserialize, Clone, Object)]
pub struct Colour {
    id: String,
    display_name: String,
    short_name: String,
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(CRUD, Debug, Serialize, Deserialize, Clone, Object)]
pub struct Cable {
    id: String,
    cross_section: f32,
    colour: Vec<Colour>,
    connected_to: Vec<Component>,
    loom: Option<Loom>,
    signal_type: Option<String>,
}

#[derive(CRUD, Debug, Serialize, Deserialize, Clone, Object)]
pub struct Loom {
    id: String,
    name: String,
    original_label: String,
}

#[derive(CRUD, Debug, Serialize, Deserialize, Clone, Object)]
pub struct Component {
    id: String,
    name: String,
    original_part_number: String,
    original_label: String,
    connections: HashMap<u16, Cable>,
    //image?
}
