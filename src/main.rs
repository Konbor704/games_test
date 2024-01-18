use bevy::{prelude::*, render::camera::ScalingMode};

const GAP_BETWEN_ENEMYS_AND_CELING: f32 = 8.0;
const GAP_BETWEN_ENEMYS_AND_WALL: f32 = 20.0;
const GAP_BETWEN_ENEMYS: f32 = 40.0;
const BOTTOM_WALL: f32 = -92.0;
const TOP_WALL: f32 = 92.0;
const LEFT_WALL: f32 = -128.0;
const RIGHT_WALL: f32 = 128.0;
const WALL_THICKNES: f32 = 1.0;
const GAP_BETWEN_ENEMYS_AND_PLAYER: f32 = 80.0;
const ENEMY_SIZE: Vec2 = Vec2::new(2.0, 2.0);
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
// const PLAYER_BOTTOM: f32 = 4.0;
const GAP_BETWEN_PLAYER_AND_FLOOR: f32 = 5.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

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

pub struct EnemyPlugin;

#[derive(Component)]
struct Player {
    speed: f32,
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Top));
    commands.spawn(WallBundle::new(WallLocation::Bottom));

    let player_y = BOTTOM_WALL + GAP_BETWEN_PLAYER_AND_FLOOR;
    let total_width_of_enemys = (RIGHT_WALL - LEFT_WALL) - 2.0 * GAP_BETWEN_ENEMYS_AND_WALL;
    let bottom_edge_of_enemys = player_y + GAP_BETWEN_ENEMYS_AND_PLAYER;
    let total_height_of_enemys = TOP_WALL - bottom_edge_of_enemys - GAP_BETWEN_ENEMYS_AND_CELING;

    assert!(total_width_of_enemys > 0.0);
    assert!(total_height_of_enemys > 0.0); 

    let n_columns = (total_width_of_enemys / (ENEMY_SIZE.x + GAP_BETWEN_ENEMYS)).floor() as usize;
    let n_rows = (total_height_of_enemys / (ENEMY_SIZE.y + GAP_BETWEN_ENEMYS)).floor() as usize;
    let vertical_gap = n_columns - 1;

    let center_of_enemys = (LEFT_WALL - RIGHT_WALL) / 2.0;
    let left_edge_of_enemys = center_of_enemys
        - (n_columns as f32 / 2.0 * ENEMY_SIZE.x)
        - vertical_gap as f32 / 2.0 * GAP_BETWEN_ENEMYS;

    let offset_x = left_edge_of_enemys + ENEMY_SIZE.x / 2.;
    let offset_y = bottom_edge_of_enemys + ENEMY_SIZE.y / 2.;

    let character = asset_server.load("character_sprite.png");

    let mut camera = Camera2dBundle::default();

    let main_player = Player {
        speed: 66.0,
    };

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
        main_player,
        Name::new("Player"),
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
                        scale: Vec3::new(ENEMY_SIZE.x, ENEMY_SIZE.y, 1.0),
                        ..default()
                    },
                    ..default()
                },
                Enemy,
                Collider,
            ));
        }
    }
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let mut movement_vector = Vec3::ZERO;

        if input.pressed(KeyCode::D) {
            movement_vector.x += 1.0;
        }
        if input.pressed(KeyCode::A) {
            movement_vector.x -= 1.0;
        }

        if movement_vector != Vec3::ZERO {
            movement_vector = movement_vector.normalize();
        }

        // Scale the movement vector by the speed and delta time
        let movement_amount = player.speed * time.delta_seconds() * movement_vector;

        // Apply the movement to the transform
        transform.translation += movement_amount;
    }
}
