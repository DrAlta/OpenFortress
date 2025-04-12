use bevy::prelude::*;

use assets::plant_data::PlantData;

use crate::Plant;

pub fn grow_system(
    time: Res<Time>, 
    plant_data: Res<PlantData>, 
    mut query: Query<&mut Plant>) 
{

    let missed_ticks = time.delta_secs() as i32 / 5;

    let ticks = 
        if (time.elapsed_secs() as i32 % 5) == 0 {
            1
        } else {
            0
        } 
        + missed_ticks;
    if  ticks > 0 {
        for mut plant in query.iter_mut() {
            let Plant { biomass_in_grams, plant_type } = plant.as_mut();

            let mut max_biomass_maybe = None;

            if let Some(groth_stages_limits) = plant_data.growth_stages_biomass_limits_in_grams.get(plant_type) {
                if let Some(max_biomass) = groth_stages_limits.last() {
                    max_biomass_maybe = Some(*max_biomass);
                };
            };


            let Some(growth_per_tick_in_grams) = plant_data.growth_per_tick_in_grams.get(plant_type) else {
                continue;
            };
            if let Some(max_biomass) = max_biomass_maybe {
                if *biomass_in_grams < max_biomass {
                    let new_biomass = *biomass_in_grams + growth_per_tick_in_grams;
                    *biomass_in_grams = 
                        if new_biomass > max_biomass {
                            max_biomass
                        } else {
                            new_biomass
                        }
                    ;
                }
            }
        }
    }
}