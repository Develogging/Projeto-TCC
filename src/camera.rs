use crate::player::Player;
use avian3d::prelude::PhysicsSet; // Usaremos para ordenar o sistema
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*; // Precisamos saber quem é o jogador

// --- Plugin ---

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera) // Spawna a câmera na inicialização
            .add_systems(
                Update,
                (
                    handle_camera_input,
                    cursor_grab_system,
                    // O sistema de atualização da câmera deve rodar DEPOIS da sincronização da física.
                    // Isso garante que a posição do jogador já foi atualizada pelo motor de física,
                    // evitando um atraso visual de um frame (one-frame lag).
                    update_camera_transform.after(PhysicsSet::Sync),
                ),
            );
    }
}

// --- Componente ---

/// Componente que armazena o estado e configuração da nossa câmera em terceira pessoa.
#[derive(Component)]
struct ThirdPersonCamera {
    /// O ponto em torno do qual a câmera orbita. Será atualizado para a posição do jogador.
    focus: Vec3,
    /// A distância da câmera ao ponto de foco (controlado pelo scroll do mouse).
    radius: f32,
    /// Rotação horizontal em radianos (controlado pelo movimento X do mouse).
    yaw: f32,
    /// Rotação vertical em radianos (controlado pelo movimento Y do mouse).
    pitch: f32,
}

impl Default for ThirdPersonCamera {
    fn default() -> Self {
        ThirdPersonCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            yaw: 0.0,
            pitch: 0.4, // Um pouco inclinado para baixo
        }
    }
}

// --- Sistemas ---

/// Spawna a entidade da câmera com nossos componentes customizados.
fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle::default(),    // O bundle padrão do Bevy para câmera 3D
        ThirdPersonCamera::default(), // Nosso componente com o estado da câmera
        Name::new("Third Person Camera"),
    ));
}

/// Sistema que lida com o input do mouse para orbitar e dar zoom.
fn handle_camera_input(
    mut camera_query: Query<&mut ThirdPersonCamera>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    windows: Query<&Window>,
) {
    let Ok(mut camera) = camera_query.get_single_mut() else {
        return;
    };
    let window = windows.single();

    // Só processa o movimento do mouse se o cursor estiver preso na janela
    if window.cursor.grab_mode == bevy::window::CursorGrabMode::Locked {
        // Rotação (órbita)
        let sensitivity = 0.002;
        for event in mouse_motion_events.read() {
            camera.yaw -= event.delta.x * sensitivity;
            camera.pitch -= event.delta.y * sensitivity;
        }
    }

    // Zoom (raio)
    let zoom_sensitivity = 0.2;
    for event in mouse_wheel_events.read() {
        camera.radius -= event.y * zoom_sensitivity;
    }

    // Limita o raio (zoom) para não ficar muito perto ou longe
    camera.radius = camera.radius.clamp(2.0, 20.0);

    // Limita o pitch para evitar que a câmera vire de cabeça para baixo
    let max_pitch = std::f32::consts::FRAC_PI_2 - 0.05; // Pouco menos que 90 graus
    camera.pitch = camera.pitch.clamp(-max_pitch, max_pitch);
}

/// Atualiza a posição e rotação (Transform) da câmera com base no estado do componente ThirdPersonCamera
/// e na posição atual do jogador.
fn update_camera_transform(
    mut camera_query: Query<(&mut Transform, &mut ThirdPersonCamera)>,
    player_query: Query<&GlobalTransform, With<Player>>,
) {
    let Ok((mut camera_transform, mut camera)) = camera_query.get_single_mut() else {
        return;
    };
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    // 1. Atualiza o ponto de foco para a posição do jogador
    // Adicionamos um pequeno offset vertical para focar no "centro" do jogador, não nos pés.
    camera.focus = player_transform.translation() + Vec3::Y * 1.0;

    // 2. Calcula a rotação da câmera com base no yaw e pitch
    let rotation = Quat::from_rotation_y(camera.yaw) * Quat::from_rotation_x(-camera.pitch);

    // 3. Calcula o offset da câmera (a posição relativa ao foco)
    // Começa com um vetor apontando para trás e o rotaciona
    let offset = rotation * (Vec3::Z * camera.radius);

    // 4. Calcula a posição final da câmera no mundo
    let final_position = camera.focus + offset;

    // 5. Atualiza o transform da câmera para que ela se posicione corretamente e olhe para o foco
    *camera_transform =
        Transform::from_translation(final_position).looking_at(camera.focus, Vec3::Y);
}

/// Sistema simples para capturar/liberar o cursor do mouse.
fn cursor_grab_system(
    mut windows: Query<&mut Window>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    key_code: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut window) = windows.get_single_mut() else {
        return;
    };

    if mouse_button.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = bevy::window::CursorGrabMode::Locked;
        window.cursor.visible = false;
    }

    if key_code.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = bevy::window::CursorGrabMode::None;
        window.cursor.visible = true;
    }
}
