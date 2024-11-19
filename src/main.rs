use bevy::prelude::*;
use bevy_tweening::lens::TransformRotationLens;
use bevy_tweening::{Animator, EaseFunction, Tween, TweenCompleted, TweeningPlugin};
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TweeningPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (keyboard_input, tween_completed))
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

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut cube_q: Query<(Entity, &mut Transform, &GlobalTransform, &mut Cube), (With<Cube>, Without<CubeChild>)>,
    mut cube_child_q: Query<(&mut Transform, &GlobalTransform), (With<CubeChild>, Without<Cube>)>,
    mut commands: Commands,
) {
    if keys.just_released(KeyCode::ArrowRight) {
        for (entity, mut transform, global_transform, mut cube) in cube_q.iter_mut() {
            if !cube.rotating {
                transform.translation.x += 0.5;
                commands.entity(entity).insert(Animator::new(
                    Tween::new(
                        EaseFunction::QuadraticIn,
                        Duration::from_millis(300),
                        TransformRotationLens {
                            start: transform.rotation,
                            end: transform.rotation
                                * Quat::from_axis_angle(global_transform.affine().inverse().transform_vector3(Vec3::Y), 90.0f32.to_radians()),
                        },
                    )
                        .with_completed_event(1),
                ));
                for (mut transform_child, global_transform_child) in cube_child_q.iter_mut() {
                    transform_child.translation += global_transform_child.affine().inverse().transform_vector3(Vec3::new(-0.5, 0.0, 0.0));
                }
                cube.rotating = true;
            }
        }
    } else if keys.just_released(KeyCode::ArrowLeft) {
        for (entity, mut transform, global_transform, mut cube) in cube_q.iter_mut() {
            if !cube.rotating {
                transform.translation.x += -0.5;
                commands.entity(entity).insert(Animator::new(
                    Tween::new(
                        EaseFunction::QuadraticIn,
                        Duration::from_millis(300),
                        TransformRotationLens {
                            start: transform.rotation,
                            end: transform.rotation
                                * Quat::from_axis_angle(global_transform.affine().inverse().transform_vector3(Vec3::Y), -90.0f32.to_radians()),
                        },
                    )
                        .with_completed_event(2),
                ));
                for (mut transform_child, global_transform_child) in cube_child_q.iter_mut() {
                    transform_child.translation += global_transform_child.affine().inverse().transform_vector3(Vec3::new(0.5, 0.0, 0.0));
                }
                cube.rotating = true;
            }
        }
    } else if keys.just_released(KeyCode::ArrowUp) {
        for (entity, mut transform, global_transform, mut cube) in cube_q.iter_mut() {
            if !cube.rotating {
                transform.translation.y += 0.5;
                commands.entity(entity).insert(Animator::new(
                    Tween::new(
                        EaseFunction::QuadraticIn,
                        Duration::from_millis(300),
                        TransformRotationLens {
                            start: transform.rotation,
                            end: transform.rotation
                                * Quat::from_axis_angle(global_transform.affine().inverse().transform_vector3(Vec3::X), -90.0f32.to_radians()),
                        },
                    )
                        .with_completed_event(3),
                ));
                for (mut transform_child, global_transform_child) in cube_child_q.iter_mut() {
                    transform_child.translation += global_transform_child.affine().inverse().transform_vector3(Vec3::new(0.0, -0.5, 0.0));
                }
                cube.rotating = true;
            }
        }
    } else if keys.just_released(KeyCode::ArrowDown) {
        for (entity, mut transform, global_transform, mut cube) in cube_q.iter_mut() {
            if !cube.rotating {
                transform.translation.y += -0.5;
                commands.entity(entity).insert(Animator::new(
                    Tween::new(
                        EaseFunction::QuadraticIn,
                        Duration::from_millis(300),
                        TransformRotationLens {
                            start: transform.rotation,
                            end: transform.rotation
                                * Quat::from_axis_angle(global_transform.affine().inverse().transform_vector3(Vec3::X), 90.0f32.to_radians()),
                        },
                    )
                        .with_completed_event(4),
                ));
                for (mut transform_child, global_transform_child) in cube_child_q.iter_mut() {
                    transform_child.translation += global_transform_child.affine().inverse().transform_vector3(Vec3::new(0.0, 0.5, 0.0));
                }
                cube.rotating = true;
            }
        }
    }
    
}

fn tween_completed(
    mut tween_completed_event: EventReader<TweenCompleted>,
    mut cube_q: Query<(&mut Transform, &mut Cube), Without<CubeChild>>,
    mut cube_child_q: Query<(&mut Transform, &GlobalTransform), (With<CubeChild>, Without<Cube>)>,
) {
    for tween_completed in tween_completed_event.read() {
        if tween_completed.user_data == 1 {
            for (mut transform, mut cube) in cube_q.iter_mut() {
                transform.translation.x += 0.5;
                cube.rotating = false;
            }
            for (mut transform_child, global_transform_child) in cube_child_q.iter_mut() {
                transform_child.translation += global_transform_child.affine().inverse().transform_vector3(Vec3::new(-0.5, 0.0, 0.0));
            }
        } else if tween_completed.user_data == 2 {
            for (mut transform, mut cube) in cube_q.iter_mut() {
                transform.translation.x += -0.5;
                cube.rotating = false;
            }
            for (mut transform_child, global_transform_child) in cube_child_q.iter_mut() {
                transform_child.translation += global_transform_child.affine().inverse().transform_vector3(Vec3::new(0.5, 0.0, 0.0));
            }
        } else if tween_completed.user_data == 3 {
            for (mut transform, mut cube) in cube_q.iter_mut() {
                transform.translation.y += 0.5;
                cube.rotating = false;
            }
            for (mut transform_child, global_transform_child) in cube_child_q.iter_mut() {
                transform_child.translation += global_transform_child.affine().inverse().transform_vector3(Vec3::new(0.0, -0.5, 0.0));
            }
        } else if tween_completed.user_data == 4 {
            for (mut transform, mut cube) in cube_q.iter_mut() {
                transform.translation.y += -0.5;
                cube.rotating = false;
            }
            for (mut transform_child, global_transform_child) in cube_child_q.iter_mut() {
                transform_child.translation += global_transform_child.affine().inverse().transform_vector3(Vec3::new(0.0, 0.5, 0.0));
            }
        }        
    }
}

#[derive(Component, Default)]
struct Cube {
    rotating: bool
}

#[derive(Component)]
struct CubeChild;
