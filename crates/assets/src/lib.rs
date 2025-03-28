use background_asset::MenuBackgroundAsset;
use bevy::prelude::*;
use dwarf_sprite::DwarfSpriteAsset;
use font_asset::FontAsset;
use resource_handles::{ResourceHandles, load_resource_assets};
use sound_assets::SoundAsset;
use ui_panel_asset::UiPanelAsset;

pub mod background_asset;
pub mod dwarf_sprite;
pub mod font_asset;
pub mod resource_handles;
pub mod sound_assets;
pub mod ui_panel_asset;

pub fn plugin(app: &mut App) {
    app.init_resource::<ResourceHandles>()
        .load_resource::<DwarfSpriteAsset>()
        .load_resource::<MenuBackgroundAsset>()
        .load_resource::<FontAsset>()
        .load_resource::<SoundAsset>()
        .load_resource::<UiPanelAsset>()
        .add_systems(PreUpdate, load_resource_assets);
}

trait LoadResource {
    /// This will load the [`Resource`] as an [`Asset`]. When all of its asset dependencies
    /// have been loaded, it will be inserted as a resource. This ensures that the resource only
    /// exists when the assets are ready.
    fn load_resource<T: Resource + Asset + Clone + FromWorld>(&mut self) -> &mut Self;
}

impl LoadResource for App {
    fn load_resource<T: Resource + Asset + Clone + FromWorld>(&mut self) -> &mut Self {
        self.init_asset::<T>();
        let world = self.world_mut();
        let value = T::from_world(world);
        let assets = world.resource::<AssetServer>();
        let handle = assets.add(value);
        let mut handles = world.resource_mut::<ResourceHandles>();
        handles
            .waiting
            .push_back((handle.untyped(), |world, handle| {
                let assets = world.resource::<Assets<T>>();
                if let Some(value) = assets.get(handle.id().typed::<T>()) {
                    world.insert_resource(value.clone());
                }
            }));
        self
    }
}
