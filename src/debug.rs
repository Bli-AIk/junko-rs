#[cfg(feature = "debug")]
mod collider;
#[cfg(feature = "debug")]
mod image_overlay;
#[cfg(feature = "debug")]
mod inspector;

use bevy::app::{App, Plugin};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, _app: &mut App) {
        #[cfg(feature = "debug")]
        {
            use inspector::debug_inspector;

            debug_inspector::setup_debug_features(_app);
            collider::debug_collider::setup_collider_debug(_app);
            image_overlay::debug_image_overlay::setup_image_overlay_debug(_app);
        }
    }
}
