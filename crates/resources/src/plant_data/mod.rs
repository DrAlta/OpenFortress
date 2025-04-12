use std::{collections::{BTreeMap, BTreeSet}, sync::Arc};

use bevy::prelude::*;

mod plant_json;
pub use plant_json::PlantJson;

pub type PlantTypeId = Arc<str>;
type Number = i32;

#[derive(serde::Deserialize)]
pub struct PlantItem {
    pub name: String,
    pub growth_per_tick_in_grams: Number,
    pub growth_stages_biomass_limits_in_grams: BTreeSet<Number>,
}

#[derive(Component)]
pub struct PlantHandle(Handle<PlantJson>);

#[derive(Resource, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlantData{
    pub growth_per_tick_in_grams: BTreeMap<PlantTypeId, Number>,
    pub growth_stages_biomass_limits_in_grams: BTreeMap<PlantTypeId, BTreeSet<Number>>,
}

impl PlantData {
    const PATH: &'static str = "plant_data.json";
}

pub fn load_json(
    asset_server: Res<AssetServer>,
    mut commands: Commands, 
) {
    let json = PlantHandle(asset_server.load(PlantData::PATH));
    commands.spawn(json);
}

pub fn process_json (
    query: Query<(Entity, &PlantHandle)>,
    mut plant_jsons: ResMut<Assets<PlantJson>>,
    mut plant_data: ResMut<PlantData>,
    mut commands: Commands, 
){
    for (entity_id, PlantHandle(handle)) in query {
        if let Some(PlantJson { plants }) = plant_jsons.remove(handle.id()) {
            for PlantItem { name, growth_per_tick_in_grams, growth_stages_biomass_limits_in_grams } in plants {
                let id: Arc<str> = Arc::from(name);
                plant_data.growth_per_tick_in_grams.insert(id.clone(), growth_per_tick_in_grams);
                plant_data.growth_stages_biomass_limits_in_grams.insert(id, growth_stages_biomass_limits_in_grams);
            }
            commands.entity(entity_id).despawn();
        }
    }
    println!("[{}:{}]process_json(): Mischisf managed!" , file!(), line!());
}
