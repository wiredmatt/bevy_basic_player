use bevy::prelude::*;
use bevy_pixel_camera::PixelCameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PixelCameraPlugin)
        .run();
}
