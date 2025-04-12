use bevy::prelude::*;

use assets::plant_data::PlantTypeId;

use crate::Number;

/// this will probably be turned into a Plant commponent to handle the growing on all plants; but for noe we are just doing carrots.
#[derive(Component, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Plant{
    pub biomass_in_grams: Number,
    pub plant_type: PlantTypeId,
}