// -------- SISTEMA DE MOVIMENTO ----------
// Separar o sistema de onde vem as entradas de movimento (teclado, rede neural, random, etc.) do sistema que faz o movimento em si, ou seja, o input se torna uma "intenção de movimento" e outro sistema cuida do movimento em si, permitindo que a fonte do movimento venha de qualquer local, que no futuro será da rede neural.
// Podemos aplicar esse conceito a outras coisas futuramente, como a intenção de investigar um som, etc.

use avian3d::prelude::*;
use bevy::prelude::*;
use rand::Rng;
// use rust_decimal::Decimal;

// --- COMPONENTES ---

/// Representa a intenção de movimento. A direção é definida por uma fonte externa
/// (jogador, IA, etc.) e depois lida por um sistema de física para aplicar a força.
#[derive(Component, Default)]
struct MovementIntent {
    direction: Vec3,
    // velocity: Decimal,
}

/// Componente marcador para a entidade do jogador.
#[derive(Component)]
struct Player;

/// Componente marcador para a entidade da criatura.
#[derive(Component)]
struct Creature;

/// Componente para gerenciar o comportamento de movimento autônomo da criatura.
/// Inclui um timer para mudar a direção do movimento periodicamente.
#[derive(Component)]
struct AutonomousMovement {
    timer: Timer,
}

// #[derive(Component)]
// struct PlayerControlled;

// --- CONSTANTES ---

const PLAYER_MOVEMENT_SPEED: f32 = 20.0; // Ajuste para a força de movimento desejada
const CREATURE_MOVEMENT_SPEED: f32 = 20.0;
const PLAYER_HEIGHT: f32 = 1.0;
const PLAYER_RADIUS: f32 = 0.4;
const CUBE_SIZE: f32 = 1.0;
const GROUND_RADIUS: f32 = 5.0;

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

/// Sistema que define a intenção de movimento para a CRIATURA.
/// Atualmente, apenas define uma direção aleatória a cada poucos segundos.
fn creature_autonomous_movement(
    time: Res<Time>,
    mut query: Query<(&mut MovementIntent, &mut AutonomousMovement), With<Creature>>,
) {
    for (mut intent, mut autonomous_state) in &mut query {
        // Atualiza o timer
        autonomous_state.timer.tick(time.delta());

        // Se o timer acabou, define uma nova direção aleatória
        if autonomous_state.timer.finished() {
            let mut rng = rand::rng();
            let random_x = rng.random_range(-1.0..1.0);
            let random_z = rng.random_range(-1.0..1.0);

            // Define uma nova direção e a normaliza
            intent.direction = Vec3::new(random_x, 0.0, random_z).normalize_or_zero();
        }
    }
}

// /// Sistema para aplicar forças ao cubo com base no MovementIntent
// fn apply_movement_intent(mut query: Query<(&MovementIntent, &mut ExternalForce), With<Player>>) {
//     if let Ok((intent, mut ext_force)) = query.single_mut() {
//         let force = intent.direction * MOVEMENT_SPEED;
//         // Define a força a ser aplicada. Como ExternalForce é persistente por padrão,
//         // definir a força para Vec3::ZERO quando não há input fará com que o
//         // LinearDamping pare o cubo.
//         ext_force.set_force(force);
//         // Se você quisesse que a força fosse aplicada apenas por um instante (impulso),
//         // você poderia usar .with_persistence(false) ao inserir o componente,
//         // ou usar ExternalImpulse. Para movimento contínuo, atualizar a força é comum.
//     }
// }

// TODO: alterar para usar ExternalForce ou algum outro tipo de força com conotações físicas.
/// Sistema genérico para aplicar forças com base no MovementIntent.
/// Funciona para qualquer entidade que tenha MovementIntent, ExternalForce, e um componente de velocidade (Player/Creature).
fn apply_movement_force(
    mut query: Query<(
        &MovementIntent,
        &mut LinearVelocity,
        Option<&Player>, // Usamos Option para diferenciar a velocidade
    )>,
) {
    for (intent, mut lin_vel, player) in &mut query {
        let speed = if player.is_some() {
            PLAYER_MOVEMENT_SPEED
        } else {
            CREATURE_MOVEMENT_SPEED
        };

        // Aplica a velocidade diretamente, o que geralmente dá um controle mais responsivo
        // do que aplicar força. A física do Avian cuidará do resto (colisões, etc.).
        lin_vel.x = intent.direction.x * speed;
        lin_vel.z = intent.direction.z * speed;
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
    // // Dynamic physics object with a collision shape and initial angular velocity
    // commands.spawn((
    //     RigidBody::Dynamic,
    //     Collider::cuboid(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE),
    //     AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
    //     ExternalForce::default(),
    //     Mesh3d(meshes.add(Cuboid::from_length(CUBE_SIZE))),
    //     MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
    //     Transform::from_xyz(0.0, 4.0, 0.0),
    //     MovementIntent::default(),
    //     Player,
    //     // // Adiciona um pouco de amortecimento linear para o cubo parar
    //     // LinearDamping(1.5),
    //     // // Bloqueia rotações para que o cubo não tombe facilmente
    //     // LockedAxes::ROTATION_LOCKED,
    // ));
    // --- JOGADOR (Cápsula controlável) ---
    commands.spawn((
        RigidBody::Dynamic,
        LinearVelocity::default(),
        LinearDamping(2.0),
        LockedAxes::ROTATION_LOCKED,
        // Usamos uma cápsula para o colisor do jogador, que navega melhor no ambiente
        Collider::capsule(PLAYER_HEIGHT / 2.0, PLAYER_RADIUS),
        Mesh3d(meshes.add(Capsule3d::new(PLAYER_RADIUS, PLAYER_HEIGHT))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))), // Cor azulada para o jogador
        Transform::from_xyz(0.0, 2.0, 0.0),
        MovementIntent::default(),
        Player,
        Name::new("Player"),
    ));
    // --- CRIATURA (Cubo autônomo) ---
    commands.spawn((
        RigidBody::Dynamic,
        // Usamos velocidade linear em vez de força externa para um controle mais direto
        LinearVelocity::default(),
        // Amortecimento para que a criatura pare quando a intenção de movimento for zero
        LinearDamping(2.0),
        // Bloqueia a rotação para evitar que o cubo tombe
        LockedAxes::ROTATION_LOCKED,
        Collider::cuboid(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE),
        Mesh3d(meshes.add(Cuboid::from_length(CUBE_SIZE))),
        MeshMaterial3d(materials.add(Color::srgb_u8(200, 100, 100))), // Cor avermelhada para a criatura
        Transform::from_xyz(2.0, 2.0, 0.0),
        MovementIntent::default(),
        Creature,
        AutonomousMovement {
            // Define um timer que termina a cada 3 segundos, para mudar de direção
            timer: Timer::from_seconds(3.0, TimerMode::Repeating),
        },
        Name::new("Creature"),
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
                creature_autonomous_movement,
                // apply_movement_intent, // Movido para FixedUpdate
            )
                .chain(),
        )
        .add_systems(FixedUpdate, apply_movement_force) // Aplica física em FixedUpdate
        .run();
}
