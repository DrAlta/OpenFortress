use bevy::prelude::*;

use resources::plant_data::PlantTypeId;

use crate::Number;

#[derive(Component, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Plant{
    pub biomass_in_grams: Number,
    pub plant_type: PlantTypeId,
}