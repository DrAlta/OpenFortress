use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy::log::LogPlugin;

use bevy_ecs_tilemap::TilemapPlugin;
#[cfg(feature = "inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[cfg(feature = "fps")]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod dwarf_plugin;
use dwarf_plugin::*;

mod map;
use map::*;

mod path;
use open_fortress::camera;
use path::*;

mod position;
use position::*;

fn main() {
    let mut app = App::new();

    #[cfg(not(feature = "debug"))]
    app.add_plugins(
        DefaultPlugins
            // to avoid blurry pixels
            .set(ImagePlugin::default_nearest()),
    );
    app.add_plugins(TilemapPlugin);
    #[cfg(feature = "debug")]
    app.add_plugins(DefaultPlugins.set(LogPlugin {
        level: bevy::log::Level::DEBUG,
        ..default()
    }));
    #[cfg(feature = "inspector")]
    app.add_plugins(WorldInspectorPlugin::new());
    #[cfg(feature = "fps")]
    app.add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default());
    app.insert_resource(Map::generate(50, 20, 10));
    app.add_plugins((DwarfPlugin, camera::plugin));
    app.add_systems(Startup, (spawn_food, spawn_map));
    app.add_systems(Update, (calculate_path, follow_path));
    app.run();
}

#[derive(Component)]
struct ClaimedBy(Entity);

#[derive(Component)]
struct Food;

fn spawn_food(
    mut commands: Commands,
    mut texture_atlasses: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("food.png");
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 8, 8, None, None);
    let texture_atlas_handle = texture_atlasses.add(texture_atlas);

    commands.spawn((
        SpriteBundle {
            texture: texture_handle.clone(),
            transform: Transform::from_translation(Position::new(5, 5, 0).into_world()),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_handle.clone(),
            index: 34,
        },
        Food,
        Name::from("Food"),
    ));
}
