use std::ops::Add;

use bevy::{core::FixedTimestep, prelude::*};

#[derive(Debug, Copy, Clone)]
enum InputCommand {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

struct OwnedInput {
    owner_id: u8,
    command: InputCommand,
}

struct PlayerConfig {
    move_speed: f32,
}

struct Player {
    id: u8,
    name: String,
}

struct GlobalInput {
    input_buffer: Vec<OwnedInput>,
}

struct InputComponent {
    commands: Vec<InputCommand>,
}

fn init_players(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    //Players
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .insert(Player {
            id: 0,
            name: "One".to_string(),
        })
        .insert(InputComponent {
            commands: Vec::new(),
        });
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut global_input: ResMut<GlobalInput>,
) {
    for keycode in keyboard_input.get_pressed() {
        match keycode {
            KeyCode::Left => global_input.input_buffer.push(OwnedInput {
                owner_id: 0,
                command: InputCommand::LEFT,
            }),
            KeyCode::Right => global_input.input_buffer.push(OwnedInput {
                owner_id: 0,
                command: InputCommand::RIGHT,
            }),
            KeyCode::Up => global_input.input_buffer.push(OwnedInput {
                owner_id: 0,
                command: InputCommand::UP,
            }),
            KeyCode::Down => global_input.input_buffer.push(OwnedInput {
                owner_id: 0,
                command: InputCommand::DOWN,
            }),
            _ => {}
        }
    }
}

fn input_system(
    mut global_input: ResMut<GlobalInput>,
    mut query: Query<(&Player, &mut InputComponent)>,
) {
    for owned_input in global_input.input_buffer.iter() {
        for (player, mut input) in query.iter_mut() {
            if owned_input.owner_id == player.id {
                input.commands.push(owned_input.command);
            }
        }
    }
    global_input.input_buffer.clear();
}

fn movement_system(
    player_config: Res<PlayerConfig>,
    mut query: Query<(&mut Transform, &mut InputComponent)>,
) {
    for (mut transform, mut input) in query.iter_mut() {
        let mut velocity = Vec3::ZERO;
        while !input.commands.is_empty() {
            let command = input.commands.pop().unwrap();
            match command {
                InputCommand::LEFT => {
                    velocity = velocity.add(Vec3::new(-1.0, 0.0, 0.0));
                }
                InputCommand::RIGHT => {
                    velocity = velocity.add(Vec3::new(1.0, 0.0, 0.0));
                }
                InputCommand::UP => {
                    velocity = velocity.add(Vec3::new(0.0, 1.0, 0.0));
                }
                InputCommand::DOWN => {
                    velocity = velocity.add(Vec3::new(0.0, -1.0, 0.0));
                }
            }
        }
        if velocity.length() > 0.0 {
            transform.translation =
                transform.translation + (velocity.normalize() * player_config.move_speed);
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(PlayerConfig { move_speed: 5.0 })
        .insert_resource(GlobalInput {
            input_buffer: Vec::new(),
        })
        .add_startup_system(init_players.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0 / 60.0))
                .with_system(keyboard_input_system.system())
                .with_system(input_system.system())
                .with_system(movement_system.system()),
        )
        .run();
}
