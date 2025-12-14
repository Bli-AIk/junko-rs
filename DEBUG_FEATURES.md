# Debug Features

This document describes the debug features available when compiling with the `debug` feature flag.

## Building with Debug Features

To enable debug features, compile with:

```bash
cargo run --features debug
```

## Available Debug Features

### F1 - Inspector Window
Opens a standalone inspector window that allows you to inspect and modify all entities, components, and resources in real-time.

- **Toggle**: Press `F1` to open/close
- The inspector window is a separate window that can be moved and resized
- When the inspector window is focused, game controls are automatically disabled to prevent interference

### F2 - Performance Monitoring
Displays real-time performance metrics including FPS, frame time, entity count, and system diagnostics.

- **Toggle**: Press `F2` to show/hide
- Metrics appear as an overlay on the main window
- Includes:
  - Frame rate (FPS)
  - Frame time
  - Entity count
  - System information
  - Render diagnostics

### F3 - Collision Visualization
Shows collision bounds for all entities with colliders (currently a placeholder for future implementation).

- **Toggle**: Press `F3` to show/hide
- Status: TODO - Implementation pending

### F4 - Debug Image Overlay
Overlays a semi-transparent debug image on top of the game window.

- **Toggle**: Press `F4` to show/hide
- Automatically loads the most recently modified image from `assets/debug/`
- Useful for comparing game output with reference images
- Supported formats: PNG, JPG, JPEG, GIF, BMP, TIFF

### F12 - Debug Help Text
Shows or hides the debug help text that lists all available debug commands.

- **Toggle**: Press `F12` to show/hide
- The help text automatically fades out after 3 seconds when first shown
- Appears in the bottom-left corner of the screen

## Debug Help Text

On startup (with debug features enabled), you will see a message in the bottom-left corner:

```
You are now running the game in Debug feature:
Inspector: [F1]
Performance monitoring: [F2]
Show colliders: [F3]
Debug image overlay: [F4]
Toggle debug help: [F12]
```

This message fades out after 3 seconds but can be recalled at any time with `F12`.

## Debug Image Overlay Usage

To use the debug image overlay feature:

1. Create a directory: `assets/debug/`
2. Place reference images in this directory
3. Press `F4` in-game to load and display the most recent image
4. The image will be displayed semi-transparently over the game window
5. Press `F4` again to hide the overlay

This is particularly useful for:
- Comparing game rendering with reference screenshots
- Pixel-perfect positioning verification
- UI layout debugging

## Implementation Notes

- All debug features are only compiled when the `debug` feature flag is enabled
- Debug features have no runtime overhead when not compiled with the flag
- The inspector uses `bevy-inspector-egui` for entity/component inspection
- Performance monitoring uses `iyes_perf_ui`
- Text animations use `bevy_tween` for smooth fade in/out effects
