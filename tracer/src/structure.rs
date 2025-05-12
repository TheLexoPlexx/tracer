use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracer_macros::{Create, Delete, Read, Update};

#[derive(Create, Read, Update, Delete, Debug, Serialize, Deserialize, Clone)]
struct Colour {
    id: String,
    display_name: String,
    short_name: String,
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Create, Read, Update, Delete, Debug, Serialize, Deserialize, Clone)]
struct Cable {
    id: String,
    cross_section: f32,
    colour: Vec<Colour>,
    connected_to: Vec<Component>,
    loom: Option<Loom>,
    signal_type: Option<String>,
}

#[derive(Create, Read, Update, Delete, Debug, Serialize, Deserialize, Clone)]
struct Loom {
    id: String,
    name: String,
    original_label: String,
}

#[derive(Create, Read, Update, Delete, Debug, Serialize, Deserialize, Clone)]
struct Component {
    id: String,
    name: String,
    original_part_number: String,
    original_label: String,
    connections: HashMap<u16, Cable>,
    //image?
}
