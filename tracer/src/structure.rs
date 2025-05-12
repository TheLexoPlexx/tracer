use serde::{
    Deserialize, Serialize,
    de::{DeserializeOwned, Error},
};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct WsRequest {
    action: String,
    data: serde_json::Value,
}

impl WsRequest {
    pub fn parse_data<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        match self.action.as_str() {
            "create" => serde_json::from_value::<JsonCreateCable>(self.data.clone()),
            "read" => serde_json::from_value::<T>(self.data.clone()),
            "update" => serde_json::from_value::<T>(self.data.clone()),
            "delete" => serde_json::from_value::<T>(self.data.clone()),
            _ => Err(serde_json::Error::custom("Invalid action")),
        }
    }
}
