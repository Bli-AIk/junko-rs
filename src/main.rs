mod debug;

use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResolution};
use debug::DebugPlugin;

const BASE_WIDTH: f32 = 320.0;
const BASE_HEIGHT: f32 = 240.0;

#[derive(Resource)]
struct WindowScale(f32);

impl Default for WindowScale {
    fn default() -> Self {
        Self(5.0)
    }
}

fn main() {
    App::new()
        .init_resource::<WindowScale>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(
                    (BASE_WIDTH * WindowScale::default().0) as u32,
                    (BASE_HEIGHT * WindowScale::default().0) as u32,
                ),
                title: "Junko-rs - Touhou LOLK Stage 6".to_string(),
                mode: WindowMode::Windowed,
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(DebugPlugin)
        .run();
}
