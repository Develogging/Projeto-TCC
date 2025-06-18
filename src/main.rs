// Declara os novos módulos para que o compilador os encontre
mod creature;
mod movement;
mod player;

use avian3d::prelude::*;
use bevy::prelude::*;
// Importa os plugins e componentes necessários dos novos módulos
use creature::{AutonomousMovement, CUBE_SIZE, Creature, CreaturePlugin};
use movement::{MovementIntent, MovementPlugin, MovementStats};
use player::{PLAYER_HEIGHT, PLAYER_RADIUS, Player, PlayerPlugin};

// --- CONSTANTES GLOBAIS ---
const GROUND_RADIUS: f32 = 20.0; // Mantendo o chão maior para dar mais espaço

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "TCC - Simulação de Ecossistema".to_string(),
                    position: WindowPosition::Centered(MonitorSelection::Index(1)),
                    resolution: (1280., 720.).into(),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            // PhysicsDebugPlugin::default(), // Descomente para visualizar colisores

            // Adiciona nossos plugins customizados. Esta parte permanece igual.
            MovementPlugin,
            PlayerPlugin,
            CreaturePlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

/// Sistema de setup, agora usando exatamente a sua estrutura de spawn original.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Chão
    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(GROUND_RADIUS, 0.1),
        Mesh3d(meshes.add(Cylinder::new(GROUND_RADIUS, 0.1))),
        MeshMaterial3d(materials.add(Color::WHITE)),
    ));

    // Jogador (Cápsula)
    commands.spawn((
        // Componentes de Física e Movimento
        RigidBody::Dynamic,
        ExternalForce::default(),
        LinearDamping(2.0),
        LockedAxes::ROTATION_LOCKED,
        Collider::capsule(PLAYER_HEIGHT / 2.0, PLAYER_RADIUS),
        // Componentes de Lógica
        MovementIntent::default(),
        MovementStats {
            speed: 5.0,
            acceleration: 15.0,
        }, // Usando a nova estrutura com aceleração
        Player,
        Name::new("Player"),
        // Componentes de Renderização e Posição
        Mesh3d(meshes.add(Capsule3d::new(PLAYER_RADIUS, PLAYER_HEIGHT))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 2.0, 0.0),
    ));

    // Criatura (Cubo)
    commands.spawn((
        // Componentes de Física e Movimento
        RigidBody::Dynamic,
        ExternalForce::default(),
        LinearDamping(2.0),
        LockedAxes::ROTATION_LOCKED,
        Collider::cuboid(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE),
        // Componentes de Lógica
        MovementIntent::default(),
        MovementStats {
            speed: 3.0,
            acceleration: 5.0,
        }, // Criatura é mais lenta e menos ágil
        Creature,
        AutonomousMovement {
            timer: Timer::from_seconds(3.0, TimerMode::Repeating),
        },
        Name::new("Creature"),
        // Componentes de Renderização e Posição
        Mesh3d(meshes.add(Cuboid::from_length(CUBE_SIZE))),
        MeshMaterial3d(materials.add(Color::srgb_u8(200, 100, 100))),
        Transform::from_xyz(2.0, 2.0, 0.0),
    ));

    // Luz
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Câmera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-10.0, 10.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
