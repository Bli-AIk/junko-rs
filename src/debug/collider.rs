#[cfg(feature = "debug")]
pub mod debug_collider {
    use bevy::prelude::*;

    #[derive(Resource, Default)]
    pub struct ColliderDebugSettings {
        pub show_colliders: bool,
    }

    pub fn setup_collider_debug(app: &mut App) {
        app.init_resource::<ColliderDebugSettings>()
            .add_systems(Update, toggle_collider_visibility_system);
    }

    fn toggle_collider_visibility_system(
        keyboard: Res<ButtonInput<KeyCode>>,
        mut settings: ResMut<ColliderDebugSettings>,
    ) {
        if keyboard.just_pressed(KeyCode::F3) {
            settings.show_colliders = !settings.show_colliders;
            info!(
                "Collider visualization: {} (TODO: Implementation pending)",
                if settings.show_colliders { "ON" } else { "OFF" }
            );
        }
    }
}
