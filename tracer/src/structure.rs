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
pub struct SignalType {
    id: String,
    name: String,
}

// Connections between Nodes
#[derive(CRUD, Debug, Serialize, Deserialize, Clone, Object)]
pub struct Cable {
    id: String,
    cross_section: f32,
    colour: Vec<Colour>,
    connected_to: Vec<Configuration>,
    loom: Option<Loom>,
    signal_type: Option<SignalType>,
}

// A Descriptor to identify a groub of Cables and their physical location
#[derive(CRUD, Debug, Serialize, Deserialize, Clone, Object)]
pub struct Loom {
    id: String,
    name: String,
    original_label: String,
}

// A Component, might be the ECU or a sensor or just a connector
#[derive(CRUD, Debug, Serialize, Deserialize, Clone, Object, Default)]
pub struct Component {
    id: String,
    name: String,
    original_part_number: String,
    original_label: String,
    pin_count: u16,
}

// A Node, An actual node in the system, differentiated from the component in case multiple of the same component are used in the system
#[derive(CRUD, Debug, Serialize, Deserialize, Clone, Object)]
pub struct Node {
    id: String,
    name: String,
    connections: HashMap<u16, Cable>,
    component: Component,
    //image?
}

#[derive(CRUD, Debug, Serialize, Deserialize, Clone, Object)]
pub struct Configuration {
    id: String,
    name: String,
    default: bool,
    nodes: Vec<Node>,
}
