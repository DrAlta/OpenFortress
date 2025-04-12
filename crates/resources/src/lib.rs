use std::collections::BTreeMap;

use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;


pub mod plant_data;

pub fn plugin(app: &mut App) {
    app
        .add_plugins(JsonAssetPlugin::<plant_data::PlantJson>::new(&["plants.json"]))
        .insert_resource(plant_data::PlantData{ growth_per_tick_in_grams: BTreeMap::new(), growth_stages_biomass_limits_in_grams: BTreeMap::new() })
        .add_systems(Startup, plant_data::load_json)
        .add_systems(PreUpdate, plant_data::process_json)
    ;
}