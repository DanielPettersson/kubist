mod board;
mod cube;
mod keyboard;
mod roll_events;

use crate::board::{BoardPlugin, BOARD_SIZE};
use crate::cube::CubePlugin;
use crate::keyboard::KeyboardPlugin;
use crate::roll_events::RollEventsPlugin;
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::prelude::{ConfigureLoadingState, LoadingState, LoadingStateAppExt};
use bevy_tweening::TweeningPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            TweeningPlugin,
            RollEventsPlugin,
            KeyboardPlugin,
            CubePlugin,
            BoardPlugin,
        ))
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::StartGame)
                .load_collection::<SceneAssets>(),
        )
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(5.0, 5.0, 10.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-7.0, -7.0, 10.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, BOARD_SIZE as f32 * 1.8).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum GameState {
    #[default]
    Loading,
    StartGame,
    InGame,
}

#[derive(AssetCollection, Resource)]
struct SceneAssets {
    #[asset(path = "models/rubiks_cube.glb#Scene0")]
    rubiks_cube: Handle<Scene>,
}
