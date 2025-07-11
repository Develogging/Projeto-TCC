use avian3d::prelude::*;
use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, apply_movement_intent);
    }
}

#[derive(Component, Default)]
pub struct MovementIntent {
    pub direction: Vec3,
}

#[derive(Component)]
pub struct MovementStats {
    pub speed: f32,
    pub acceleration: f32,
}

fn apply_movement_intent(
    mut query: Query<(
        &MovementIntent,
        &MovementStats,
        &LinearVelocity,
        &mut ExternalForce,
    )>,
) {
    for (intent, stats, lin_vel, mut ext_force) in &mut query {
        let target_velocity = intent.direction * stats.speed;
        let velocity_diff = target_velocity - lin_vel.0;
        let force = velocity_diff * stats.acceleration;
        ext_force.set_force(Vec3::new(force.x, 0.0, force.z));
    }
}
