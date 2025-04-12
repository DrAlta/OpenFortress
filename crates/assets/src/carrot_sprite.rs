use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

#[derive(Asset, Clone, Resource, TypePath)]
pub struct CarrotSpriteAsset {
    pub sprite: Handle<Image>,
    pub texture_atlas: TextureAtlas,
}

impl CarrotSpriteAsset {
    const PATH: &'static str = "Carrot Sprite Sheet 1.0v.png";
}

impl FromWorld for CarrotSpriteAsset {
    fn from_world(world: &mut World) -> Self {
        let layout_handle = {
            let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 4, 1, None, None);
            let mut layouts = world.resource_mut::<Assets<TextureAtlasLayout>>();
            layouts.add(layout)
        };
        let assets = world.resource::<AssetServer>();
        CarrotSpriteAsset {
            sprite: assets.load_with_settings(
                CarrotSpriteAsset::PATH,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            texture_atlas: TextureAtlas {
                index: 0,
                layout: layout_handle,
            },
        }
    }
}
