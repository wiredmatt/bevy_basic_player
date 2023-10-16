use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};
use bevy_asset_loader::prelude::*;
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};
use player::PlayerPlugin;

pub mod player;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default)]
enum Direction {
    #[default]
    Right,
    Left,
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
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Game".into(),
                        present_mode: PresentMode::AutoVsync,
                        canvas: Some("#game".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        mode: WindowMode::Windowed,
                        focused: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(PixelCameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_systems(OnEnter(GameState::Next), setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(PixelCameraBundle::from_resolution(320, 240, false));

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
