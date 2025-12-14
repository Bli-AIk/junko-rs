#[cfg(feature = "debug")]
pub mod debug_image_overlay {
    use bevy::prelude::*;
    use std::fs;
    use std::path::Path;
    use std::time::SystemTime;

    #[derive(Resource, Default)]
    pub struct ImageOverlaySettings {
        pub show_overlay: bool,
    }

    #[derive(Component)]
    pub struct DebugImageOverlay;

    pub fn setup_image_overlay_debug(app: &mut App) {
        app.init_resource::<ImageOverlaySettings>().add_systems(
            Update,
            (toggle_image_overlay_system, maintain_overlay_system),
        );
    }

    fn toggle_image_overlay_system(
        keyboard: Res<ButtonInput<KeyCode>>,
        mut settings: ResMut<ImageOverlaySettings>,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        overlay_query: Query<Entity, With<DebugImageOverlay>>,
        window_query: Query<&Window>,
    ) {
        if keyboard.just_pressed(KeyCode::F4) {
            settings.show_overlay = !settings.show_overlay;

            if settings.show_overlay {
                for entity in overlay_query.iter() {
                    commands.entity(entity).despawn();
                }

                if let Some(latest_image_path) = find_latest_debug_image() {
                    info!("Loading debug overlay image: {}", latest_image_path);

                    let image_handle: Handle<Image> = asset_server.load(&latest_image_path);

                    if let Ok(window) = window_query.single() {
                        let window_width = window.width();
                        let window_height = window.height();

                        commands
                            .spawn((
                                Name::new("DebugImageOverlay"),
                                DebugImageOverlay,
                                Node {
                                    position_type: PositionType::Absolute,
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    top: Val::Px(0.0),
                                    left: Val::Px(0.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.3)),
                                ZIndex(1000),
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    ImageNode {
                                        image: image_handle,
                                        color: Color::srgba(1.0, 1.0, 1.0, 0.7),
                                        ..default()
                                    },
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                        max_width: Val::Px(window_width),
                                        max_height: Val::Px(window_height),
                                        ..default()
                                    },
                                ));
                            });

                        info!("Debug image overlay: ON");
                    }
                }
            } else {
                info!("Debug image overlay: OFF");
            }
        }
    }

    fn maintain_overlay_system(
        settings: Res<ImageOverlaySettings>,
        mut commands: Commands,
        overlay_query: Query<Entity, With<DebugImageOverlay>>,
    ) {
        if !settings.show_overlay {
            for entity in overlay_query.iter() {
                commands.entity(entity).despawn();
            }
        }
    }

    fn find_latest_debug_image() -> Option<String> {
        let possible_paths = ["assets/debug"];
        let extensions = ["png", "jpg", "jpeg", "gif", "bmp", "tiff"];

        let mut latest_file: Option<(String, SystemTime)> = None;
        let mut found_debug_folder = false;

        for debug_path in &possible_paths {
            if !Path::new(debug_path).exists() {
                continue;
            }

            found_debug_folder = true;

            if let Ok(entries) = fs::read_dir(debug_path) {
                for entry in entries.flatten() {
                    if let Ok(file_type) = entry.file_type()
                        && file_type.is_file()
                        && let Some(file_name) = entry.file_name().to_str()
                        && let Some(extension) = file_name.split('.').next_back()
                        && extensions.contains(&extension.to_lowercase().as_str())
                        && let Ok(metadata) = entry.metadata()
                        && let Ok(modified) = metadata.modified()
                    {
                        let relative_path = format!("debug/{}", file_name);

                        if latest_file.is_none() || latest_file.as_ref().unwrap().1 < modified {
                            latest_file = Some((relative_path, modified));
                        }
                    }
                }
                if latest_file.is_some() {
                    break;
                }
            }
        }

        if !found_debug_folder {
            warn!("Debug folder not found in any of the expected locations");
            return None;
        }

        if let Some((path, _)) = latest_file {
            info!("Selected latest debug image: {}", path);
            Some(path)
        } else {
            warn!("No image files found in debug folder");
            None
        }
    }
}
