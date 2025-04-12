pub use bevy::prelude::*;

use assets::carrot_sprite::CarrotSpriteAsset;

use crate::Plant;

pub fn on_add_carrot(
    trigger: Trigger<OnAdd, Plant>,
    carrot: Res<CarrotSpriteAsset>,
    mut commands: Commands,
) {
    commands.entity(trigger.target()).insert((
        Name::new("Carrot"),
        Sprite {
            image: carrot.sprite.clone_weak(),
            texture_atlas: Some(carrot.texture_atlas.clone()),
            ..default()
        },
        Plant{ biomass_in_grams: 10 , plant_type: crate::carrot_id()},
        //Plant,
    ));
}
