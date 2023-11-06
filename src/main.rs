//! Illustrates how to change window settings and shows how to affect
//! the mouse pointer in various ways.

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    window::{CompositeAlphaMode, CursorGrabMode, PresentMode, WindowLevel, WindowMode},
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: WindowMode::Windowed,
                        title: "I am a window!".into(),
                        resolution: (55., 68.).into(),
                        position: WindowPosition::At(IVec2 { x: 0, y: 1080 }),
                        decorations: false,
                        present_mode: PresentMode::AutoVsync,
                        resize_constraints: WindowResizeConstraints {
                            min_width: 55.,
                            min_height: 68.,
                            max_width: 55.,
                            max_height: 68.,
                        },
                        transparent: true,
                        composite_alpha_mode: CompositeAlphaMode::Auto,
                        ..default()
                    }),
                    ..default()
                }),
        ) // prevents blurry sprites
        // .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(ClearColor(Color::rgba(0.0, 0.0, 0.0, 0.0))) // Transparent black
        .add_startup_system(setup)
        .add_system(animate_sprite)
        // .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_system(change_title)
        // .add_system(toggle_vsync)
        .add_system(toggle_cursor)
        .add_system(cycle_cursor_icon)
        .add_system(switch_level)
        .run();
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut windows: Query<&mut Window>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut Transform,
    )>,
) {
    // let window = windows.get_primary().expect("No primary window found.");
    let mut window = windows.single_mut();

    for (indices, mut timer, mut sprite, mut _transform) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let time_elapased = time.elapsed().as_secs_f32().round() * 1.0;
            let mut move_x_pixel = time_elapased % 1920.0;

            let move_right_or_left = time_elapased as i32 / 1920;
            if move_right_or_left % 2 == 0 {
                sprite.flip_x = false;
            } else {
                sprite.flip_x = true;
                move_x_pixel = 1920.0 - move_x_pixel;
            }

            window.position = WindowPosition::At(IVec2 {
                // x: 200,
                x: move_x_pixel as i32,
                y: 1080,
            });

            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // let texture_handle = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let texture_handle = asset_server.load("megumin.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(271.0, 340.0), 12, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 11 };
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            // transform: Transform::from_scale(Vec3::splat(2.0)),
            transform: Transform::from_scale(Vec3::splat(0.2)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.11, TimerMode::Repeating)),
        // AnimationTimer(Timer::from_seconds(0.09, TimerMode::Repeating)),
    ));
}

fn switch_level(_input: Res<Input<KeyCode>>, mut windows: Query<&mut Window>) {
    // if input.just_pressed(KeyCode::T) {
    if 1 > 0 {
        let mut window = windows.single_mut();

        window.window_level = WindowLevel::AlwaysOnTop;
        // window.window_level = match window.window_level {
        //     WindowLevel::AlwaysOnBottom => WindowLevel::Normal,
        //     WindowLevel::Normal => WindowLevel::AlwaysOnTop,
        //     WindowLevel::AlwaysOnTop => WindowLevel::AlwaysOnBottom,
        // };
        // info!("WINDOW_LEVEL: {:?}", window.window_level);
    }
}

/// This system will then change the title during execution
fn change_title(mut windows: Query<&mut Window>, time: Res<Time>) {
    let mut window = windows.single_mut();
    window.title = format!(
        "Seconds since startup: {}",
        time.elapsed().as_secs_f32().round()
    );
}

fn toggle_cursor(mut windows: Query<&mut Window>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        let mut window = windows.single_mut();

        window.cursor.visible = !window.cursor.visible;
        window.cursor.grab_mode = match window.cursor.grab_mode {
            CursorGrabMode::None => CursorGrabMode::Locked,
            CursorGrabMode::Locked | CursorGrabMode::Confined => CursorGrabMode::None,
        };
    }
}

/// This system cycles the cursor's icon through a small set of icons when clicking
fn cycle_cursor_icon(
    mut windows: Query<&mut Window>,
    input: Res<Input<MouseButton>>,
    mut index: Local<usize>,
) {
    let mut window = windows.single_mut();

    const ICONS: &[CursorIcon] = &[
        CursorIcon::Default,
        CursorIcon::Hand,
        CursorIcon::Wait,
        CursorIcon::Text,
        CursorIcon::Copy,
    ];

    if input.just_pressed(MouseButton::Left) {
        *index = (*index + 1) % ICONS.len();
    } else if input.just_pressed(MouseButton::Right) {
        *index = if *index == 0 {
            ICONS.len() - 1
        } else {
            *index - 1
        };
    }

    window.cursor.icon = ICONS[*index];
}
