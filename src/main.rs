// -------- SISTEMA DE MOVIMENTO ----------
// Separar o sistema de onde vem as entradas de movimento (teclado, rede neural, random, etc.) do sistema que faz o movimento em si, ou seja, o input se torna uma "intenção de movimento" e outro sistema cuida do movimento em si, permitindo que a fonte do movimento venha de qualquer local, que no futuro será da rede neural.
// Podemos aplicar esse conceito a outras coisas futuramente, como a intenção de investigar um som, etc.

use avian3d::prelude::*;
use bevy::prelude::*;
// use rust_decimal::Decimal;

#[derive(Component, Default)]
struct MovementIntent {
    direction: Vec3,
    // velocity: Decimal,
}

const MOVEMENT_SPEED: f32 = 20.0; // Ajuste para a força de movimento desejada
const CUBE_SIZE: f32 = 1.0;
const GROUND_RADIUS: f32 = 5.0;

#[derive(Component)]
struct Player;

// #[derive(Component)]
// struct PlayerControlled;

/// Sistema para ler o input do teclado e atualizar o MovementIntent
fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut MovementIntent, With<Player>>,
    camera_query: Query<&GlobalTransform, With<Camera3d>>, // Para movimento relativo à câmera
) {
    if let Ok(mut intent) = query.single_mut() {
        let mut direction_vector = Vec3::ZERO;

        // Pega a orientação da câmera (apenas rotação no eixo Y para movimento no plano XZ)
        // Se você quiser um movimento totalmente relativo à câmera 3D, a lógica será mais complexa
        // Esta é uma simplificação para movimento no plano XZ relativo à direção da câmera.
        let camera_transform = camera_query.single().unwrap_or_else(|_| {
            // Fallback se não houver câmera (improvável em um jogo real)
            warn!("Nenhuma câmera 3D encontrada, usando direções do mundo.");
            &GlobalTransform::IDENTITY
        });

        // Pega o vetor "para frente" da câmera projetado no plano XZ
        let mut forward: Vec3 = camera_transform.forward().into();
        forward.y = 0.0;
        forward = forward.normalize_or_zero();

        // Pega o vetor "para a direita" da câmera projetado no plano XZ
        let mut right: Vec3 = camera_transform.right().into();
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

        // Normaliza o vetor de direção para que o movimento diagonal não seja mais rápido
        intent.direction = direction_vector.normalize_or_zero();
    }
}

/// Sistema para aplicar forças ao cubo com base no MovementIntent
fn apply_movement_intent(mut query: Query<(&MovementIntent, &mut ExternalForce), With<Player>>) {
    if let Ok((intent, mut ext_force)) = query.single_mut() {
        let force = intent.direction * MOVEMENT_SPEED;
        // Define a força a ser aplicada. Como ExternalForce é persistente por padrão,
        // definir a força para Vec3::ZERO quando não há input fará com que o
        // LinearDamping pare o cubo.
        ext_force.set_force(force);
        // Se você quisesse que a força fosse aplicada apenas por um instante (impulso),
        // você poderia usar .with_persistence(false) ao inserir o componente,
        // ou usar ExternalImpulse. Para movimento contínuo, atualizar a força é comum.
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Static physics object with a collision shape
    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(GROUND_RADIUS, 0.1),
        Mesh3d(meshes.add(Cylinder::new(GROUND_RADIUS, 0.1))),
        MeshMaterial3d(materials.add(Color::WHITE)),
    ));
    // Dynamic physics object with a collision shape and initial angular velocity
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE),
        AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
        ExternalForce::default(),
        Mesh3d(meshes.add(Cuboid::from_length(CUBE_SIZE))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 4.0, 0.0),
        MovementIntent::default(),
        Player,
        // // Adiciona um pouco de amortecimento linear para o cubo parar
        // LinearDamping(1.5),
        // // Bloqueia rotações para que o cubo não tombe facilmente
        // LockedAxes::ROTATION_LOCKED,
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy no Segundo Monitor!".to_string(),
                    // Define a posição para ser centralizada no segundo monitor (índice 1)
                    position: WindowPosition::Centered(MonitorSelection::Index(1)),
                    // Você também pode tentar posicionar em coordenadas absolutas se souber
                    // as coordenadas do seu segundo monitor no desktop virtual.
                    // Exemplo: position: WindowPosition::At(IVec2::new(1920, 0)), // Se o 2º monitor começa em x=1920
                    resolution: (1280., 720.).into(), // Defina a resolução desejada
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    // Outras configurações...
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(), // Adiciona os plugins de física do Avian
                                       // PhysicsDebugPlugin::default(), // Descomente para visualizar colisores
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_input,
                // apply_movement_intent, // Movido para FixedUpdate
            )
                .chain(),
        )
        .add_systems(FixedUpdate, apply_movement_intent) // Aplica física em FixedUpdate
        .run();
}
