use crate::movement::MovementIntent;
use bevy::prelude::*; // Importando do nosso novo módulo

// --- Plugin ---

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // O sistema de input do jogador é adicionado aqui.
        // Ele roda no Update, pois inputs devem ser capturados o mais rápido possível.
        app.add_systems(Update, handle_keyboard_input);
    }
}

// --- Componentes ---

/// Componente marcador para a entidade do jogador.
#[derive(Component)]
pub struct Player;

// --- Constantes Específicas do Jogador ---

pub const PLAYER_HEIGHT: f32 = 1.0;
pub const PLAYER_RADIUS: f32 = 0.4;

// --- Sistemas ---

/// Sistema para ler o input do teclado e atualizar o MovementIntent do jogador.
/// (Código movido de main.rs, sem alteração de lógica)
fn handle_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut MovementIntent, With<Player>>,
    camera_query: Query<&GlobalTransform, With<Camera3d>>,
) {
    if let Ok(mut intent) = query.single_mut() {
        let Ok(camera_transform) = camera_query.single() else {
            return;
        };

        let mut direction_vector = Vec3::ZERO;

        let mut forward = *camera_transform.forward();
        forward.y = 0.0;
        forward = forward.normalize_or_zero();

        let mut right = *camera_transform.right();
        right.y = 0.0;
        right = right.normalize_or_zero();

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction_vector += forward;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction_vector -= forward;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction_vector -= right;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction_vector += right;
        }

        intent.direction = direction_vector.normalize_or_zero();
    }
}
