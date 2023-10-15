use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_pixel_camera::PixelCameraPlugin;

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
        .add_plugins(DefaultPlugins)
        .add_plugins(PixelCameraPlugin)
        .run();
}
