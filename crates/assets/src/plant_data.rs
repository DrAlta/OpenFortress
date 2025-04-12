use std::{collections::{BTreeMap, BTreeSet}, sync::Arc};

use bevy::prelude::*;

pub type PlantTypeId = Arc<str>;
type Number = i32;

#[derive(Resource, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlantData{
    pub growth_per_tick_in_grams: BTreeMap<PlantTypeId, Number>,
    pub growth_stages_biomass_limits_in_grams: BTreeMap<PlantTypeId, BTreeSet<Number>>,
}

need to add loading the data from a json file