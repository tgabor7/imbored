use std::ops::Sub;

use crate::components::{
    acceleration::Acceleration, main_camera::MainCamera, physics::Physics, player::Player,
    projectile::Projectile,
};
use bevy::{prelude::*, render::camera, window::PrimaryWindow};
use bevy_rapier2d::prelude::{
        ActiveEvents, Ccd, Collider, ExternalForce, GravityScale, KinematicCharacterController, Restitution, RigidBody, Sensor, Sleeping, Velocity
    };

pub fn player_movement(
    mut commands: Commands,
    mut controllers: Query<&mut KinematicCharacterController>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<(&mut Transform, &Camera, &GlobalTransform), With<MainCamera>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    player: Query<(&Transform, &Player), Without<MainCamera>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    for mut controller in controllers.iter_mut() {
        let mut direction = Vec2::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 10.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 10.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 10.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 10.0;
        }

        controller.translation = Some(direction);

        let player_transform = player.single();

        let camera_speed = 0.1;
        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.0.translation = camera_transform.0.translation.lerp(
                Vec3::new(
                    player_transform.0.translation.x,
                    player_transform.0.translation.y,
                    camera_transform.0.translation.z,
                ),
                camera_speed,
            );

            let mut cursor_position = Vec2::new(0.0, 0.0);

            // There is only one primary window, so we can similarly get it from the query:
            let window = q_windows.single();

            let (camera, camera_transform_g) = q_camera.single();
            // check if the cursor is inside the window and get its position
            // then, ask bevy to convert into world coordinates, and truncate to discard Z
            if let Some(world_position) = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform_g, cursor))
                .map(|ray| ray.origin.truncate())
            {
                cursor_position = world_position;
            }

            let direction = cursor_position.sub(Vec2::new(
                player_transform.0.translation.x,
                player_transform.0.translation.y,
            ));

            let projectile_initial_velocity = Velocity {
                linvel: direction.normalize() * 1000.0,
                angvel: 0.2,
            };

            if mouse_buttons.pressed(MouseButton::Left) {

                commands
                    .spawn(RigidBody::Dynamic)
                    .insert(TransformBundle::from(Transform::from_translation(
                        player_transform.0.translation + direction.normalize().extend(0.0) * 30.0,
                    )))
                    .insert(projectile_initial_velocity)
                    .insert(GravityScale(0.5))
                    .insert(Collider::cuboid(10.0, 10.0))
                    .insert(Sleeping::disabled())
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(Projectile { life: 1.0, damage: 1 })
                    .insert(Ccd::enabled());
            }
        }
    }
}
