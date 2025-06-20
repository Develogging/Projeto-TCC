use crate::player::Player;
use avian3d::prelude::PhysicsSet;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow; // Precisamos disso para pegar a janela principal de forma robusta

// --- Plugin ---

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera).add_systems(
            Update,
            (
                handle_camera_input,
                // O sistema de cursor agora está correto para Bevy 0.16
                cursor_grab_system,
                // A ordenação do sistema para evitar "one-frame lag" continua correta
                update_camera_transform.after(PhysicsSet::Sync),
            ),
        );
    }
}

// --- Componente ---

/// Componente que armazena o estado e configuração da nossa câmera em terceira pessoa.
/// (Nenhuma mudança necessária aqui)
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

// --- Sistemas ---

/// Spawna a entidade da câmera com nossos componentes customizados.
/// (Nenhuma mudança necessária aqui)
fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        ThirdPersonCamera::default(),
        Name::new("Third Person Camera"),
    ));
}

/// Sistema que lida com o input do mouse para orbitar e dar zoom.
/// Foi atualizado para usar a query da janela primária.
fn handle_camera_input(
    mut camera_query: Query<&mut ThirdPersonCamera>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    // API MODERNA: Pegamos a janela através de uma query com o marcador PrimaryWindow
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(mut camera) = camera_query.single_mut() else {
        return;
    };
    let Ok(window) = window_query.single() else {
        return;
    };

    // A lógica interna aqui já estava correta, verificando o grab_mode.
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

/// Atualiza o Transform da câmera com base na posição do jogador.
/// (Nenhuma mudança necessária aqui)
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

/// Sistema para capturar/liberar o cursor do mouse.
/// *** ESTE É O SISTEMA CORRIGIDO ***
fn cursor_grab_system(
    // API MODERNA: Em vez de `Query<&mut Window>`, usamos `Query<&mut Window, With<PrimaryWindow>>`
    // para garantir que estamos modificando a janela principal.
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    key_code: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut window) = window_query.single_mut() else {
        return;
    };

    if mouse_button.just_pressed(MouseButton::Left) {
        // API MODERNA (0.15+): Modificamos os campos `grab_mode` e `visible` do recurso `Cursor`
        // que está dentro da entidade `Window`.
        window.cursor_options.grab_mode = bevy::window::CursorGrabMode::Locked;
        window.cursor_options.visible = false;
    }

    if key_code.just_pressed(KeyCode::Escape) {
        // API MODERNA (0.15+): Restauramos os valores para liberar o cursor.
        window.cursor_options.grab_mode = bevy::window::CursorGrabMode::None;
        window.cursor_options.visible = true;
    }
}
