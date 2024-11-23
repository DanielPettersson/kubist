use crate::roll_events::RollEvent;
use bevy::app::{App, Update};
use bevy::hierarchy::Children;
use bevy::math::Quat;
use bevy::prelude::{default, BuildChildren, Commands, Component, Entity, EventReader, GlobalTransform, Handle, InheritedVisibility, Plugin, Query, ResMut, Resource, Scene, SceneBundle, Startup, Transform, TransformBundle, Vec3, With, Without};
use bevy_tweening::lens::TransformRotationLens;
use bevy_tweening::{Animator, EaseFunction, Tween, TweenCompleted};
use std::time::Duration;

pub struct CubePlugin;

impl Plugin for CubePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init)
            .add_systems(Update, (roll, roll_completed));
    }
}

fn init(mut commands: Commands) {
    commands.insert_resource(RollingCubesCounter(0));
}

pub fn spawn_cube(commands: &mut Commands, cube_child_scene_handle: Handle<Scene>, x: f32, y: f32) -> Entity {
    commands
        .spawn((
            TransformBundle::from_transform(Transform::from_xyz(x, y, 0.0)),
            InheritedVisibility::default(),
            Cube::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                SceneBundle {
                    scene: cube_child_scene_handle,
                    transform: Transform::from_translation(Vec3::new(0.0, -0.5, 0.5))
                        .with_scale(Vec3::splat(1. / 6.)),
                    ..default()
                },
                CubeChild { scale: 1. / 6. },
            ));
        }).id()
}

fn roll(
    mut roll_events: EventReader<RollEvent>,
    mut rolling_cubes_counter: ResMut<RollingCubesCounter>,
    mut cube_q: Query<(Entity, &mut Transform, &GlobalTransform, &Children), With<Cube>>,
    mut cube_child_q: Query<(&mut Transform, &GlobalTransform, &CubeChild), Without<Cube>>,
    mut commands: Commands,
) {
    for roll_event in roll_events.read() {
        if rolling_cubes_counter.0 == 0 {
            if let Ok((entity, mut transform, global_transform, children)) =
                cube_q.get_mut(*roll_event.get_entity())
            {
                let roll_translation = roll_event.roll_translation();
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
                                        .transform_vector3(roll_event.roll_axis()),
                                    90.0f32.to_radians(),
                                ),
                        },
                    )
                    .with_completed_event(roll_event.get_id()),
                ));
                for &child in children.iter() {
                    translate_child(&mut cube_child_q, child, roll_translation);
                }
                rolling_cubes_counter.0 += 1;
            }
        }
    }
}

fn roll_completed(
    mut rolling_cubes_counter: ResMut<RollingCubesCounter>,
    mut tween_completed_event: EventReader<TweenCompleted>,
    mut cube_q: Query<(&mut Transform, &Children), Without<CubeChild>>,
    mut cube_child_q: Query<(&mut Transform, &GlobalTransform, &CubeChild), Without<Cube>>,
) {
    for tween_completed in tween_completed_event.read() {
        let roll_event = RollEvent::from_id(tween_completed.user_data, tween_completed.entity);
        let roll_trans = roll_event.roll_translation();

        if let Ok((mut transform, children)) = cube_q.get_mut(tween_completed.entity) {
            transform.translation += roll_trans;
            rolling_cubes_counter.0 -= 1;

            for &child in children.iter() {
                translate_child(&mut cube_child_q, child, roll_trans);
            }
        }
    }
}

fn translate_child(
    cube_child_q: &mut Query<(&mut Transform, &GlobalTransform, &CubeChild), Without<Cube>>,
    entity: Entity,
    roll_trans: Vec3,
) {
    if let Ok((mut transform_child, global_transform_child, cube_child)) =
        cube_child_q.get_mut(entity)
    {
        transform_child.translation -= global_transform_child
            .affine()
            .inverse()
            .transform_vector3(roll_trans)
            * cube_child.scale;
    }
}

#[derive(Component, Default)]
struct Cube;

#[derive(Component)]
struct CubeChild {
    scale: f32,
}

#[derive(Resource)]
pub struct RollingCubesCounter(pub usize);
