use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};

#[derive(Component)]
struct AnimationTimer(Timer);

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
} // `Player` will act as a Tag, for us to identify its entity.

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default)]
enum Direction {
    #[default]
    Right,
    Left,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default)]
enum PlayerState {
    #[default]
    Idle,
    Walk,
    Run,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Loading,
    Next,
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_loading_state(LoadingState::new(GameState::Loading).continue_to_state(GameState::Next))
        .add_collection_to_loading_state::<_, PlayerAssets>(GameState::Loading)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PixelCameraPlugin)
        .add_systems(OnEnter(GameState::Next), setup)
        .add_systems(
            Update,
            (animate, handle_movement_input).run_if(in_state(GameState::Next)),
        )
        .run();
}

fn setup(mut commands: Commands, assets: Res<PlayerAssets>) {
    commands.spawn(PixelCameraBundle::from_resolution(320, 240, true));

    commands.spawn((Player {
        state: PlayerState::Idle,
        facing: Direction::Right
    },SpriteSheetBundle {
        transform: Transform {
            translation: Vec3::new(0., 0., 0.),
            ..Default::default()
        }, // the SpriteSheet Bundle gives the `Transform` component, that's why we can use it in line 55
        sprite: TextureAtlasSprite::new(0), // `sprite` here is the default image to show while not playing an animation.
        texture_atlas: assets.animations.clone(),
        ..Default::default()
    }, AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating))));

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0., 0., 0.),
            custom_size: Some(Vec2::new(1000.0, 10.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., -15., 0.)),
        ..default()
    });
}

fn handle_movement_input(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player), With<Player>>,
) {
    let (mut player_position, mut player_state) = query.single_mut();

    let mut speed = 0.;

    let base_speed = 50. * time.delta_seconds();

    let bonus_speed = if input.pressed(KeyCode::ShiftLeft) {
        60. * time.delta_seconds()
    } else {
        0.
    };

    if input.pressed(KeyCode::Right) {
        player_state.facing = Direction::Right;
        speed = base_speed + bonus_speed;
    } else if input.pressed(KeyCode::Left) {
        player_state.facing = Direction::Left;
        speed = -(base_speed + bonus_speed);
    }

    if speed != 0. {
        // NOTE: assigning these in this exact order will ensure the animation starts playing just before the character moves, otherwise you'll get a weird look

        player_state.state = if bonus_speed != 0. {
            PlayerState::Run
        } else {
            PlayerState::Walk
        };

        player_position.translation.x += speed;
    } else {
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
        }
    }
}
