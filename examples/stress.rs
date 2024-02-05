//! A simplified version of the "many sprites" Bevy example, now with metrics.

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_metrics_dashboard::{
    CoreMetricsPlugin, DashboardPlugin, DashboardWindow, RegistryPlugin, RenderMetricsPlugin,
};
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(RegistryPlugin::default())
        .add_plugins(CoreMetricsPlugin)
        .add_plugins(RenderMetricsPlugin)
        .add_plugins(DashboardPlugin)
        .add_systems(Startup, (create_dashboard, setup))
        .add_systems(Update, move_camera)
        .run();
}

fn create_dashboard(mut commands: Commands) {
    commands.spawn(DashboardWindow::new("Metrics Dashboard"));
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let mut rng = rand::thread_rng();

    let tile_size = Vec2::splat(64.0);
    let map_size = Vec2::splat(320.0);

    let half_x = (map_size.x / 2.0) as i32;
    let half_y = (map_size.y / 2.0) as i32;

    let sprite_handle = assets.load("icon.png");

    commands.spawn(Camera2dBundle::default());

    // Builds and spawns the sprites
    let mut sprites = vec![];
    for y in -half_y..half_y {
        for x in -half_x..half_x {
            let position = Vec2::new(x as f32, y as f32);
            let translation = (position * tile_size).extend(rng.gen::<f32>());
            let rotation = Quat::from_rotation_z(rng.gen::<f32>());
            let scale = Vec3::splat(rng.gen::<f32>() * 2.0);

            sprites.push(SpriteBundle {
                texture: sprite_handle.clone(),
                transform: Transform {
                    translation,
                    rotation,
                    scale,
                },
                sprite: Sprite {
                    custom_size: Some(tile_size),
                    color: Color::WHITE,
                    ..default()
                },
                ..default()
            });
        }
    }
    commands.spawn_batch(sprites);
}

fn move_camera(time: Res<Time>, mut camera_query: Query<&mut Transform, With<Camera>>) {
    let mut camera_transform = camera_query.single_mut();
    camera_transform.rotate_z(time.delta_seconds() * 0.1);
}
