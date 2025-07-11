mod camera;
mod creature;
mod movement;
mod player;

use avian3d::prelude::*;
use bevy::prelude::*;
use camera::CameraPlugin;
use creature::{AutonomousMovement, CUBE_SIZE, Creature, CreaturePlugin};
use movement::{MovementIntent, MovementPlugin, MovementStats};
use player::{PLAYER_HEIGHT, PLAYER_RADIUS, Player, PlayerPlugin};

const GROUND_RADIUS: f32 = 20.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "TCC - Simulação de Ecossistema".to_string(),
                    position: WindowPosition::Centered(MonitorSelection::Index(1)),
                    // resolution: (1280., 720.).into(),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            MovementPlugin,
            PlayerPlugin,
            CreaturePlugin,
            CameraPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

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
        // Física e movimento
        RigidBody::Dynamic,
        ExternalForce::default(),
        LinearDamping(2.0),
        LockedAxes::ROTATION_LOCKED,
        Collider::capsule(PLAYER_RADIUS / 2.0, PLAYER_HEIGHT),
        // Lógica
        MovementIntent::default(),
        MovementStats {
            speed: 5.0,
            acceleration: 15.0,
        },
        Player,
        Name::new("Player"),
        // Renderização e posição
        Mesh3d(meshes.add(Capsule3d::new(PLAYER_RADIUS, PLAYER_HEIGHT))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 2.0, 0.0),
    ));

    // Criatura (Cubo)
    commands.spawn((
        // Física e movimento
        RigidBody::Dynamic,
        ExternalForce::default(),
        LinearDamping(2.0),
        LockedAxes::ROTATION_LOCKED,
        Collider::cuboid(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE),
        // Lógica
        MovementIntent::default(),
        MovementStats {
            speed: 3.0,
            acceleration: 5.0,
        },
        Creature,
        AutonomousMovement {
            timer: Timer::from_seconds(3.0, TimerMode::Repeating),
        },
        Name::new("Creature"),
        // Renderização e posição
        Mesh3d(meshes.add(Cuboid::from_length(CUBE_SIZE))),
        MeshMaterial3d(materials.add(Color::srgb_u8(200, 100, 100))),
        Transform::from_xyz(2.0, 2.0, 0.0),
        DebugRender::default().with_collider_color(Color::srgb(1.0, 0.0, 0.0)),
    ));

    // Luz
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}
