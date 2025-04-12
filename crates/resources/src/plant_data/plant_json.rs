use bevy::{asset::Asset, reflect::TypePath};

use super::PlantItem;

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct PlantJson {
    pub plants: Vec<PlantItem>,
}

