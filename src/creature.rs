use crate::movement::MovementIntent;
// use avian3d::{
//     math::{AdjustPrecision, Scalar, Vector},
//     prelude::{SpatialQuery, SpatialQueryFilter},
// };
use bevy::prelude::*;
use rand::Rng;
// use std::f32::consts::PI; // Importando do nosso novo módulo

// --- Plugin ---

pub struct CreaturePlugin;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        // Sistema de movimento autônomo (placeholder) é adicionado aqui.
        app.add_systems(
            Update,
            creature_autonomous_movement, /*creature_vision_system*/
        );
    }
}

// --- Componentes ---

/// Componente marcador para a entidade da criatura.
#[derive(Component)]
pub struct Creature;

/// Componente para gerenciar o comportamento de movimento autônomo da criatura.
#[derive(Component)]
pub struct AutonomousMovement {
    pub timer: Timer,
}

// /// Armazena os parâmetros e os dados de saída do sistema de visão da criatura.
// #[derive(Component, Debug)]
// pub struct CreatureVision {
//     pub range: f32,
//     pub angle: f32,
//     pub ray_count: u32,
//     pub readings: Vec<f32>,
// }

// impl Default for CreatureVision {
//     fn default() -> Self {
//         let ray_count = 11;
//         Self {
//             range: 8.0,
//             angle: PI / 2.0, // 90 graus
//             ray_count,
//             readings: vec![f32::MAX; ray_count as usize],
//         }
//     }
// }

// --- Constantes Específicas da Criatura ---
pub const CUBE_SIZE: f32 = 1.0;

// --- Sistemas ---

/// Sistema que define a intenção de movimento para a CRIATURA.
/// (Código movido de main.rs, sem alteração de lógica)
fn creature_autonomous_movement(
    time: Res<Time>,
    mut query: Query<(&mut MovementIntent, &mut AutonomousMovement), With<Creature>>,
) {
    for (mut intent, mut autonomous_state) in &mut query {
        autonomous_state.timer.tick(time.delta());

        if autonomous_state.timer.finished() {
            let mut rng = rand::rng();
            let random_x = rng.random_range(-1.0..=1.0);
            let random_z = rng.random_range(-1.0..=1.0);

            intent.direction = Vec3::new(random_x, 0.0, random_z).normalize_or_zero();
        }
    }
}

// fn creature_vision_system(
//     query: SpatialQuery,
//     mut creature_query: Query<(Entity, &mut CreatureVision, &GlobalTransform), With<Creature>>,
//     mut gizmos: Gizmos,
// ) {
//     let origin = Vector::new(-200.0, 2.0, 0.0);
//     let direction = Dir3::X;
//     let filter = SpatialQueryFilter::default();

//     if let Some(ray_hit_data) =
//         query.cast_ray_predicate(origin, direction, Scalar::MAX, true, &filter, &|entity| {
//             println!("{:?}", creature_query.get(entity));
//             true
//         })
//     {
//         // Set the length of the ray indicator to look more like a laser,
//         let contact_point = (origin + direction.adjust_precision() * ray_hit_data.distance).x;
//         let target_scale = 1000.0 + contact_point * 2.0;
//         println!("{}, {}", contact_point, target_scale);
//     }
// }
