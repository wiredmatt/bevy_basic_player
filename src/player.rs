use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{Direction, GameState};

#[derive(Component)]
struct AnimationTimer(Timer);

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default)]
enum PlayerState {
    #[default]
    Idle,
    Walk,
    Run,
    Jump,
    Land,
}

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(texture_atlas(
        tile_size_x = 48.,
        tile_size_y = 48.,
        columns = 10,
        rows = 7,
        padding_y = 2.
    ))]
    #[asset(path = "player/spritesheet.png")]
    animations: Handle<TextureAtlas>,
}

#[derive(Component)]
struct Player {
    state: PlayerState,
    facing: Direction,
    horizontal_speed: f32,
    vertical_speed: f32,
} // `Player` will act as a Tag, for us to identify its entity.

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_collection_to_loading_state::<_, PlayerAssets>(GameState::Loading)
            .add_systems(OnEnter(GameState::Next), setup)
            .add_systems(
                Update,
                (handle_movement_input, animate, mock_gravity, idle_fallback)
                    .run_if(in_state(GameState::Next)),
            );
    }
}

fn setup(mut commands: Commands, assets: Res<PlayerAssets>) {
    commands.spawn((Player {
      state: PlayerState::Idle,
      facing: Direction::Right,
      horizontal_speed: 0.0,
      vertical_speed: 0.0,
  },SpriteSheetBundle {
      transform: Transform {
          translation: Vec3::new(0., 0., 0.),
          ..Default::default()
      },
      sprite: TextureAtlasSprite::new(0), // `sprite` here is the default image to show while not playing an animation.
      texture_atlas: assets.animations.clone(),
      ..Default::default()
  }, AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating))));
}

fn handle_movement_input(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player), With<Player>>,
) {
    let (mut player_position, mut player_state) = query.single_mut();

    let mut horizontal_speed = 0.;
    let mut vertical_speed = player_state.vertical_speed;

    let base_speed = 50. * time.delta_seconds();

    let bonus_speed = if input.pressed(KeyCode::ShiftLeft) {
        60. * time.delta_seconds()
    } else {
        0.
    };

    if input.pressed(KeyCode::Right) {
        player_state.facing = Direction::Right;
        horizontal_speed = base_speed + bonus_speed;
    } else if input.pressed(KeyCode::Left) {
        player_state.facing = Direction::Left;
        horizontal_speed = -(base_speed + bonus_speed);
    }

    // if we want to jump, and are not already jumping / mid jump
    if input.just_pressed(KeyCode::Space)
        && vertical_speed == 0.0
        && player_position.translation.y == 0.0
    {
        vertical_speed = (330. * time.delta_seconds()).clamp(0., 5.); // if we don't clamp this, we'll get varying jump heights.
    }

    player_state.horizontal_speed = horizontal_speed;
    player_state.vertical_speed = vertical_speed;

    if vertical_speed != 0. && player_state.state != PlayerState::Land {
        if vertical_speed > (-200.) * time.delta_seconds() {
            player_state.state = PlayerState::Jump;
        } else {
            player_state.state = PlayerState::Land;
        }
    }

    // if we're moving, and not jumping, play the walk / run animations.
    if horizontal_speed != 0. && vertical_speed == 0. {
        // NOTE: assigning these in this exact order will ensure the animation starts playing just before the character moves, otherwise you'll get a weird look

        player_state.state = if bonus_speed != 0. {
            PlayerState::Run
        } else {
            PlayerState::Walk
        };
    }

    player_position.translation.x += horizontal_speed;
    player_position.translation.y += vertical_speed;
}

fn mock_gravity(time: Res<Time>, mut query: Query<(&mut Player, &mut Transform), With<Player>>) {
    let (mut player_state, mut transform) = query.single_mut();

    if transform.translation.y > 0. {
        player_state.vertical_speed -= (300. / 25.) * time.delta_seconds();
    } else {
        player_state.vertical_speed = 0.;
        transform.translation.y = 0.;
    }
}

fn idle_fallback(mut query: Query<&mut Player, With<Player>>) {
    let mut player_state = query.single_mut();

    let is_not_moving = player_state.horizontal_speed == 0.0;

    let is_not_jumping = player_state.vertical_speed == 0.0;

    if is_not_moving && is_not_jumping {
        player_state.state = PlayerState::Idle;
    }
}

fn animate(
    time: Res<Time>,
    mut query: Query<(&Player, &mut AnimationTimer, &mut TextureAtlasSprite), With<Player>>,
) {
    let (player, mut timer, mut sprite) = query.single_mut();

    match player.facing {
        Direction::Left => {
            sprite.flip_x = true;
        }
        Direction::Right => {
            sprite.flip_x = false;
        }
    } // flip the sprite first to avoid a moonwalk effect

    timer.0.tick(time.delta());
    if timer.0.finished() {
        match player.state {
            PlayerState::Idle => {
                sprite.index = (sprite.index + 1) % 10; // Idle animation goes from frame 0 to 10, check spritesheet
            }
            PlayerState::Walk => {
                sprite.index = 10 + (sprite.index + 1) % 8; // Walk animation goes from frame 10 to 18
            }
            PlayerState::Run => {
                sprite.index = 50 + (sprite.index + 1) % 8; // Run animation goes from frame 50 to 58
            }
            PlayerState::Jump => {
                if sprite.index != 32 {
                    sprite.index = 30 + (sprite.index + 1) % 3; // Jump animation goes from frame 30 to 32
                }
            }
            PlayerState::Land => {
                sprite.index = 41; // i didn't like the landing animation, a static frame looked better imo
            }
        }
    }
}
