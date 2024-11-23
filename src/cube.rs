use crate::roll_events::RollEvent;
use crate::{GameState, SceneAssets};
use bevy::app::{App, Update};
use bevy::hierarchy::Children;
use bevy::math::Quat;
use bevy::prelude::{default, BuildChildren, Commands, Component, Entity, EventReader, GlobalTransform, InheritedVisibility, OnEnter, Plugin, Query, Res, SceneBundle, Transform, TransformBundle, Vec3, With, Without};
use bevy_tweening::lens::TransformRotationLens;
use bevy_tweening::{Animator, EaseFunction, Tween, TweenCompleted};
use std::time::Duration;

pub struct CubePlugin;

impl Plugin for CubePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_cube)
            .add_systems(Update, (roll, tween_completed));
    }
}
pub fn spawn_cube(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
) {
    commands
        .spawn((
            TransformBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
            InheritedVisibility::default(),
            Cube::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                SceneBundle {
                    scene: scene_assets.rubiks_cube.clone(),
                    transform: Transform::from_translation(Vec3::new(0.0, -0.5, 0.5)).with_scale(Vec3::splat(1. / 6.)),
                    ..default()
                },
                CubeChild {
                    scale: 1. / 6.
                },
            ));
        });
}

fn roll(
    mut roll_events: EventReader<RollEvent>,
    mut cube_q: Query<
        (
            Entity,
            &mut Transform,
            &GlobalTransform,
            &mut Cube,
            &Children,
        ),
        With<Cube>,
    >,
    mut cube_child_q: Query<(&mut Transform, &GlobalTransform, &CubeChild), Without<Cube>>,
    mut commands: Commands,
) {
    for roll_event in roll_events.read() {
        for (entity, mut transform, global_transform, mut cube, children) in cube_q.iter_mut() {
            if !cube.rotating {
                let roll_translation = roll_translation(roll_event);
                transform.translation += roll_translation;
                commands.entity(entity).insert(Animator::new(
                    Tween::new(
                        EaseFunction::QuadraticIn,
                        Duration::from_millis(300),
                        TransformRotationLens {
                            start: transform.rotation,
                            end: transform.rotation
                                * Quat::from_axis_angle(
                                    global_transform
                                        .affine()
                                        .inverse()
                                        .transform_vector3(roll_axis(roll_event)),
                                    90.0f32.to_radians(),
                                ),
                        },
                    )
                    .with_completed_event(roll_event.get_id()),
                ));
                for &child in children.iter() {
                    if let Ok((mut transform_child, global_transform_child, cube_child)) =
                        cube_child_q.get_mut(child)
                    {
                        transform_child.translation -= global_transform_child
                            .affine()
                            .inverse()
                            .transform_vector3(roll_translation) * cube_child.scale;
                    }
                }
                cube.rotating = true;
            }
        }
    }
}

fn roll_translation(roll_event: &RollEvent) -> Vec3 {
    match roll_event {
        RollEvent::Right => Vec3::new(0.5, 0.0, 0.0),
        RollEvent::Left => Vec3::new(-0.5, 0.0, 0.0),
        RollEvent::Up => Vec3::new(0.0, 0.5, 0.0),
        RollEvent::Down => Vec3::new(0.0, -0.5, 0.0),
    }
}

fn roll_axis(roll_event: &RollEvent) -> Vec3 {
    match roll_event {
        RollEvent::Right => Vec3::Y,
        RollEvent::Left => Vec3::NEG_Y,
        RollEvent::Up => Vec3::NEG_X,
        RollEvent::Down => Vec3::X,
    }
}

fn tween_completed(
    mut tween_completed_event: EventReader<TweenCompleted>,
    mut cube_q: Query<(&mut Transform, &mut Cube), Without<CubeChild>>,
    mut cube_child_q: Query<(&mut Transform, &GlobalTransform, &CubeChild), Without<Cube>>,
) {
    for tween_completed in tween_completed_event.read() {
        let roll_event = RollEvent::from_id(tween_completed.user_data);
        let roll_trans = roll_translation(&roll_event);
        for (mut transform, mut cube) in cube_q.iter_mut() {
            transform.translation += roll_trans;
            cube.rotating = false;
        }
        for (mut transform_child, global_transform_child, cube_child) in cube_child_q.iter_mut() {
            transform_child.translation -= global_transform_child
                .affine()
                .inverse()
                .transform_vector3(roll_trans) * cube_child.scale;
        }
    }
}

#[derive(Component, Default)]
struct Cube {
    rotating: bool,
}

#[derive(Component)]
struct CubeChild {
    scale: f32,
}
