use bevy::{prelude::*, render::camera::ScalingMode};
use enemys::EnemyPlugin;

mod enemys;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_plugins(EnemyPlugin)
        .run();
}

#[derive(Component)]
struct Player {
    speed: f32,
    health: f32,
    armor: f32,
}

impl Player {
    fn effective_health(&self) {
        let mut e_health = self.health * self.armor;
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let character = asset_server.load("character_sprite.png");

    let mut camera = Camera2dBundle::default();

    let main_player = Player {
        speed: 66.0,
        health: 100.0,
        armor: 2.0,
    };

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: (256.0),
        min_height: (192.0),
    };

    commands.spawn(camera);

    commands.spawn((
        SpriteBundle {
            texture: character,
            transform: Transform::from_xyz(0.0, -80.0, 0.0),
            ..default()
        },
        // Player {
        //     speed: 50.0,
        //     health: 100.0,
        //     armor: 2.0,
        // },
        main_player,
        Name::new("Player"),
    ));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let mut movement_vector = Vec3::ZERO;

        // if input.pressed(KeyCode::W) {
        //     movement_vector.y += 1.0;
        // }
        // if input.pressed(KeyCode::S) {
        //     movement_vector.y -= 1.0;
        // }
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
