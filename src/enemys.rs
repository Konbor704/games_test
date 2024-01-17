use bevy::prelude::*;

pub struct EnemyPlugin;

const GAP_BETWEN_ENEMYS_AND_CELING: f32 = 20.0;
const GAP_BETWEN_ENEMYS_AND_WALL: f32 = 20.0;
const GAP_BETWEN_ENEMYS: f32 = 40.0;
pub const BOTTOM_WALL: f32 = -92.0;
pub const TOP_WALL: f32 = 92.0;
pub const LEFT_WALL: f32 = -128.0;
pub const RIGHT_WALL: f32 = 128.0;
pub const WALL_THICKNES: f32 = 1.0;
pub const GAP_BETWEN_ENEMYS_AND_PLAYER: f32 = 80.0;
const ENEMY_SIZE: Vec2 = Vec2::new(16.0, 16.0);
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const PLAYER_BOTTOM: f32 = 4.0;

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

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup) // spawn_enemy_parent
            // .add_systems(Update, spawn_enemy)
            .register_type::<Enemy>();
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Top));
    commands.spawn(WallBundle::new(WallLocation::Bottom));

    let total_width_of_enemys = (RIGHT_WALL - LEFT_WALL) - 2.0 * GAP_BETWEN_ENEMYS_AND_WALL;
    let bottom_edge_of_enemys = PLAYER_BOTTOM + GAP_BETWEN_ENEMYS_AND_PLAYER;
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

// fn spawn_enemy_parent(mut commands: Commands) {
//     commands.spawn((
//         SpatialBundle::default(),
//         EnemyParent,
//         Name::new("Enemy Parent"),
//     ));
// }

// fn spawn_enemy(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     parent: Query<Entity, With<EnemyParent>>,
// ) {
//     let texture = asset_server.load("enemy_ship.png");
//     let parent = parent.single();
//
//     commands.spawn((
//         SpriteBundle {
//             texture,
//             ..default()
//         },
//         Enemy { health: 2.0 },
//         Name::new("Enemy_l1"),
//     ));
// }
