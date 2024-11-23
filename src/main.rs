mod keyboard;
mod roll_events;

use crate::keyboard::KeyboardPlugin;
use crate::roll_events::{RollEvent, RollEventsPlugin};
use bevy::prelude::*;
use bevy_tweening::lens::TransformRotationLens;
use bevy_tweening::{Animator, EaseFunction, Tween, TweenCompleted, TweeningPlugin};
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            TweeningPlugin,
            RollEventsPlugin,
            KeyboardPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (roll, tween_completed))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            TransformBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
            InheritedVisibility::default(),
            Cube::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                    material: materials.add(Color::srgb_u8(124, 144, 255)),
                    transform: Transform::from_xyz(0.0, 0.0, 0.5),
                    ..default()
                },
                CubeChild,
            ));
        });

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

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, -10.0, 10.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
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
    mut cube_child_q: Query<(&mut Transform, &GlobalTransform), Without<Cube>>,
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
                    if let Ok((mut transform_child, global_transform_child)) =
                        cube_child_q.get_mut(child)
                    {
                        transform_child.translation -= global_transform_child
                            .affine()
                            .inverse()
                            .transform_vector3(roll_translation);
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
    mut cube_child_q: Query<(&mut Transform, &GlobalTransform), (With<CubeChild>, Without<Cube>)>,
) {
    for tween_completed in tween_completed_event.read() {
        let roll_event = RollEvent::from_id(tween_completed.user_data);
        let roll_trans = roll_translation(&roll_event);
        for (mut transform, mut cube) in cube_q.iter_mut() {
            transform.translation += roll_trans;
            cube.rotating = false;
        }
        for (mut transform_child, global_transform_child) in cube_child_q.iter_mut() {
            transform_child.translation -= global_transform_child
                .affine()
                .inverse()
                .transform_vector3(roll_trans);
        }
    }
}

#[derive(Component, Default)]
struct Cube {
    rotating: bool,
}

#[derive(Component)]
struct CubeChild;
