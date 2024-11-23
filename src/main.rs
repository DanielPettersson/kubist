mod board;
mod cube;
mod keyboard;
mod roll_events;

use crate::board::BoardPlugin;
use crate::cube::CubePlugin;
use crate::keyboard::KeyboardPlugin;
use crate::roll_events::RollEventsPlugin;
use bevy::core_pipeline::motion_blur::{MotionBlur, MotionBlurBundle};
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(
            Vec3::new(0.0, 0.0, 1.0),
            Vec2::new(10.0, 10.0),
        )),
        material: materials.add(Color::srgb_u8(255, 127, 127)),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(10.0, 10.0, 10.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-10.0, -10.0, 10.0),
        ..default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 7.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        },
        MotionBlurBundle {
            motion_blur: MotionBlur {
                shutter_angle: 1.0,
                samples: 3,
            },
            ..default()
        },
    ));
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
