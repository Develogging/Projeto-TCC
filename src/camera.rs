use crate::player::Player;
use avian3d::prelude::PhysicsSet;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera).add_systems(
            Update,
            (
                handle_camera_input,
                cursor_grab_system,
                update_camera_transform.after(PhysicsSet::Sync),
            ),
        );
    }
}

#[derive(Component)]
struct ThirdPersonCamera {
    focus: Vec3,
    radius: f32,
    yaw: f32,
    pitch: f32,
}

impl Default for ThirdPersonCamera {
    fn default() -> Self {
        ThirdPersonCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            yaw: 0.0,
            pitch: 0.4,
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        ThirdPersonCamera::default(),
        Name::new("Third Person Camera"),
    ));
}

fn handle_camera_input(
    mut camera_query: Query<&mut ThirdPersonCamera>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(mut camera) = camera_query.single_mut() else {
        return;
    };
    let Ok(window) = window_query.single() else {
        return;
    };

    if window.cursor_options.grab_mode == bevy::window::CursorGrabMode::Locked {
        let sensitivity = 0.002;
        for event in mouse_motion_events.read() {
            camera.yaw -= event.delta.x * sensitivity;
            // camera.pitch -= event.delta.y * sensitivity; // Mouse para cima -> câmera para cima
            camera.pitch += event.delta.y * sensitivity; // Mouse para cima -> câmera para baixo
        }
    }

    let zoom_sensitivity = 0.2;
    for event in mouse_wheel_events.read() {
        camera.radius -= event.y * zoom_sensitivity;
    }

    camera.radius = camera.radius.clamp(2.0, 20.0);
    let max_pitch = std::f32::consts::FRAC_PI_2 - 0.05;
    camera.pitch = camera.pitch.clamp(-max_pitch, max_pitch);
}

fn update_camera_transform(
    mut camera_query: Query<(&mut Transform, &mut ThirdPersonCamera)>,
    player_query: Query<&GlobalTransform, With<Player>>,
) {
    let Ok((mut camera_transform, mut camera)) = camera_query.single_mut() else {
        return;
    };
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    camera.focus = player_transform.translation() + Vec3::Y * 1.0;
    let rotation = Quat::from_rotation_y(camera.yaw) * Quat::from_rotation_x(-camera.pitch);
    let offset = rotation * (Vec3::Z * camera.radius);
    let final_position = camera.focus + offset;

    *camera_transform =
        Transform::from_translation(final_position).looking_at(camera.focus, Vec3::Y);
}

fn cursor_grab_system(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    key_code: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut window) = window_query.single_mut() else {
        return;
    };

    if mouse_button.just_pressed(MouseButton::Left) {
        window.cursor_options.grab_mode = bevy::window::CursorGrabMode::Locked;
        window.cursor_options.visible = false;
    }

    if key_code.just_pressed(KeyCode::Escape) {
        window.cursor_options.grab_mode = bevy::window::CursorGrabMode::None;
        window.cursor_options.visible = true;
    }
}
