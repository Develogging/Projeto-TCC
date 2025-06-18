use avian3d::prelude::*;
use bevy::prelude::*;

// --- Plugin ---

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        // O sistema de aplicação de movimento é genérico e pertence a este plugin.
        // Ele deve rodar no FixedUpdate para consistência com a física.
        app.add_systems(FixedUpdate, apply_movement_intent);
    }
}

// --- Componentes (Exportados publicamente com `pub`) ---

/// Representa a intenção de movimento. A direção é definida por uma fonte externa.
#[derive(Component, Default)]
pub struct MovementIntent {
    pub direction: Vec3,
}

/// Define os atributos de movimento de uma entidade.
#[derive(Component)]
pub struct MovementStats {
    /// A velocidade máxima que a entidade deseja atingir.
    pub speed: f32,
    /// A força de aceleração. Um valor maior resultará em uma resposta mais rápida.
    pub acceleration: f32,
}

// --- Sistemas ---

/// Sistema genérico para aplicar FORÇAS com base no MovementIntent.
/// Este sistema agora implementa uma lógica mais física.
fn apply_movement_intent(
    mut query: Query<(
        &MovementIntent,
        &MovementStats,
        &LinearVelocity, // Precisamos ler a velocidade atual
        &mut ExternalForce,
    )>,
) {
    for (intent, stats, lin_vel, mut ext_force) in &mut query {
        // 1. Calcular a velocidade alvo
        let target_velocity = intent.direction * stats.speed;

        // 2. Calcular a diferença entre a velocidade alvo e a atual (apenas no plano XZ)
        let velocity_diff = target_velocity - lin_vel.0;

        // 3. Calcular a força necessária para corrigir essa diferença
        // A força é proporcional à diferença de velocidade e à aceleração da entidade.
        // Isso cria um movimento mais suave e com inércia.
        let force = velocity_diff * stats.acceleration;

        // 4. Aplicar a força
        // Usamos set_force para garantir que esta seja a única força que nosso sistema de movimento aplica.
        ext_force.set_force(Vec3::new(force.x, 0.0, force.z));
    }
}
