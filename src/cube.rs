use crate::roll_events::RollEvent;
use bevy::app::{App, Update};
use bevy::math::Quat;
use bevy::prelude::{default, BuildChildren, Commands, Component, Entity, EventReader, GlobalTransform, Handle, InheritedVisibility, OnEnter, Plugin, Query, Res, ResMut, Resource, Scene, SceneBundle, Startup, Transform, TransformBundle, Vec3, With};
use bevy_tweening::lens::{TransformPositionLens, TransformRotationLens};
use bevy_tweening::{Animator, BoxedTweenable, EaseFunction, Tracks, Tween, TweenCompleted};
use std::time::Duration;
use EaseFunction::{QuadraticIn, QuadraticInOut, QuadraticOut};
use crate::GameState;

pub struct CubePlugin;

impl Plugin for CubePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init)
            .add_systems(Update, (roll, roll_completed)).add_systems(OnEnter(GameState::InGame), enter_game);
    }
}

fn init(mut commands: Commands) {
    commands.insert_resource(RollingCubesCounter(0));
    commands.insert_resource(RollDuration::default());
}

fn enter_game(mut roll_duration: ResMut<RollDuration>) {
    *roll_duration = RollDuration::default();
}

pub fn spawn_cube(
    commands: &mut Commands,
    cube_child_scene_handle: Handle<Scene>,
    x: f32,
    y: f32,
) -> Entity {
    commands
        .spawn((
            TransformBundle::from_transform(Transform::from_xyz(x, y, 0.5)),
            InheritedVisibility::default(),
            Cube::default(),
        ))
        .with_children(|parent| {
            parent.spawn((SceneBundle {
                scene: cube_child_scene_handle,
                transform: Transform::from_translation(Vec3::new(0.0, -0.5, 0.0))
                    .with_scale(Vec3::splat(1. / 6.)),
                ..default()
            },));
        })
        .id()
}

fn roll(
    mut commands: Commands,
    mut roll_events: EventReader<RollEvent>,
    mut rolling_cubes_counter: ResMut<RollingCubesCounter>,
    mut cube_q: Query<(Entity, &Transform, &GlobalTransform), With<Cube>>,
    roll_duration: Res<RollDuration>,
) {
    for roll_event in roll_events.read() {
        if rolling_cubes_counter.0 == 0 {
            if let Ok((entity, transform, global_transform)) =
                cube_q.get_mut(*roll_event.get_entity())
            {
                let roll_translation = roll_event.roll_translation();
                let half_duration = Duration::from_millis((roll_duration.0.as_millis() / 2) as u64);

                commands.entity(entity).insert(Animator::new(Tracks::new([
                    Tween::new(
                        QuadraticOut,
                        half_duration,
                        TransformPositionLens {
                            start: transform.translation,
                            end: transform.translation + roll_translation * 0.5 + Vec3::Z * 0.5,
                        },
                    )
                    .then(Tween::new(
                        QuadraticIn,
                        half_duration,
                        TransformPositionLens {
                            start: transform.translation + roll_translation * 0.5 + Vec3::Z * 0.5,
                            end: transform.translation + roll_translation,
                        },
                    ))
                    .into(),
                    Box::new(
                        Tween::new(
                            QuadraticInOut,
                            roll_duration.0,
                            TransformRotationLens {
                                start: transform.rotation,
                                end: transform.rotation
                                    * Quat::from_axis_angle(
                                        global_transform
                                            .affine()
                                            .inverse()
                                            .transform_vector3(roll_event.roll_axis()),
                                        90.0f32.to_radians(),
                                    ),
                            },
                        )
                        .with_completed_event(roll_event.get_id()),
                    ) as BoxedTweenable<Transform>,
                ])));
                rolling_cubes_counter.0 += 1;
            }
        }
    }
}

fn roll_completed(
    mut rolling_cubes_counter: ResMut<RollingCubesCounter>,
    mut tween_completed_event: EventReader<TweenCompleted>,
) {
    for _ in tween_completed_event.read() {
        rolling_cubes_counter.0 -= 1;
    }
}

#[derive(Component, Default)]
struct Cube;

#[derive(Resource)]
pub struct RollingCubesCounter(pub usize);

#[derive(Resource)]
pub struct RollDuration(pub Duration);

impl Default for RollDuration {
    fn default() -> Self {
        Self(Duration::from_millis(300))
    }
}
