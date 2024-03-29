use bevy::{prelude::*, render::camera::ScalingMode, sprite::MaterialMesh2dBundle};

const GAP_BETWEN_ENEMYS_AND_CELING: f32 = 10.0;
const GAP_BETWEN_ENEMYS_AND_WALL: f32 = 10.0;
const GAP_BETWEN_ENEMYS: f32 = 20.0;
const BOTTOM_WALL: f32 = -95.0;
const TOP_WALL: f32 = 95.0;
const LEFT_WALL: f32 = -128.0;
const RIGHT_WALL: f32 = 128.0;
const WALL_THICKNES: f32 = 5.0;
const GAP_BETWEN_ENEMYS_AND_PLAYER: f32 = 80.0;
const ENEMY_SIZE: Vec2 = Vec2::new(1.0, 1.0);
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const PLAYER_SPEED: f32 = 100.0;
const PLAYER_BOTTOM: f32 = 4.0;
const GAP_BETWEN_PLAYER_AND_FLOOR: f32 = 10.0;
const BULLET_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_event::<CollisionEvent>()
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (apply_velocity, move_player).chain())
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

pub struct EnemyPlugin;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Bullet;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Component)]
struct Collider;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EnemyParent;

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;

        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNES, arena_width + WALL_THICKNES)
            }
            WallLocation::Top | WallLocation::Bottom => {
                Vec2::new(arena_width + WALL_THICKNES, WALL_THICKNES)
            }
        }
    }
}

impl WallBundle {
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let player_y = BOTTOM_WALL + GAP_BETWEN_PLAYER_AND_FLOOR;
    let total_width_of_enemys = (RIGHT_WALL - LEFT_WALL) - 2.0 * GAP_BETWEN_ENEMYS_AND_WALL;
    let bottom_edge_of_enemys = player_y + GAP_BETWEN_ENEMYS_AND_PLAYER;
    let total_height_of_enemys = TOP_WALL - bottom_edge_of_enemys - GAP_BETWEN_ENEMYS_AND_CELING;

    assert!(total_width_of_enemys > 0.0);
    assert!(total_height_of_enemys > 0.0);

    let n_columns = (total_width_of_enemys / (ENEMY_SIZE.x + GAP_BETWEN_ENEMYS)).floor() as usize;
    let n_rows = (total_height_of_enemys / (ENEMY_SIZE.y + GAP_BETWEN_ENEMYS)).floor() as usize;
    let vertical_gap = n_columns - 1;

    let center_of_enemys = (LEFT_WALL + RIGHT_WALL) / 2.0;
    let left_edge_of_enemys = center_of_enemys
        - (n_columns as f32 / 2.0 * ENEMY_SIZE.x)
        - vertical_gap as f32 / 2.0 * GAP_BETWEN_ENEMYS;

    let offset_x = left_edge_of_enemys + ENEMY_SIZE.x / 2.;
    let offset_y = bottom_edge_of_enemys + ENEMY_SIZE.y / 2.;

    let character = asset_server.load("character_sprite.png");

    let mut camera = Camera2dBundle::default();

    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Top));
    commands.spawn(WallBundle::new(WallLocation::Bottom));

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: (256.0),
        min_height: (192.0),
    };

    commands.spawn(camera);

    commands.spawn((
        SpriteBundle {
            texture: character,
            transform: Transform::from_xyz(0.0, player_y, 0.0),
            ..default()
        },
        Player,
        Name::new("Player"),
        Collider,
    ));

    let texture = asset_server.load("enemy_ship.png");

    for row in 0..n_rows {
        for column in 0..n_columns {
            let enemy_position = Vec2::new(
                offset_x + column as f32 * (ENEMY_SIZE.x + GAP_BETWEN_ENEMYS),
                offset_y + row as f32 * (ENEMY_SIZE.y + GAP_BETWEN_ENEMYS),
            );

            commands.spawn((
                SpriteBundle {
                    texture: texture.clone(),
                    transform: Transform {
                        translation: enemy_position.extend(0.0),
                        scale: Vec3::new(ENEMY_SIZE.x, ENEMY_SIZE.y, 0.5),
                        ..default()
                    },
                    ..default()
                },
                Enemy,
                Collider,
            ));
        }
    }

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(BULLET_COLOR)),
            ..default()
        },
        Bullet,
    ));
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::A) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::D) {
        direction += 1.0;
    }

    // Calculate the new horizontal paddle position based on player input
    let new_paddle_position =
        paddle_transform.translation.x + direction * PLAYER_SPEED * time.delta_seconds();

    // Update the paddle position,
    // making sure it doesn't cause the paddle to leave the arena
    let left_bound = LEFT_WALL + WALL_THICKNES / 2.0 + 16.0 / 2.0 + 10.0;
    let right_bound = RIGHT_WALL - WALL_THICKNES / 2.0 - 16.0 / 2.0 - PLAYER_BOTTOM;

    paddle_transform.translation.x = new_paddle_position.clamp(left_bound, right_bound);
}
fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn shooting(
    commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
) {
}
