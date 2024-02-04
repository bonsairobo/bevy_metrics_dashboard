use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_metrics_dashboard::{dashboard_window::DashboardWindow, DashboardPlugin};
use metrics::{
    counter, describe_counter, describe_gauge, describe_histogram, gauge, histogram, Unit,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(DashboardPlugin::default())
        .add_systems(Startup, (describe_metrics, create_dashboard))
        .add_systems(Update, update_metrics)
        .run();
}

// Just needs to happen after the DashboardPlugin is added.
//
// NOTE: If you want to describe counters **before** creating your
// Bevy app, see DashboardPlugin::with_registry.
fn describe_metrics() {
    describe_counter!(
        "num_frames",
        Unit::Count,
        "The number of frames rendered since app start"
    );
    describe_gauge!(
        "frame_time",
        Unit::Milliseconds,
        "Frame latency in milliseconds"
    );
    describe_histogram!(
        "frame_time",
        Unit::Milliseconds,
        "Frame latency in milliseconds"
    );
}

fn create_dashboard(mut commands: Commands) {
    commands.spawn(DashboardWindow::new("Metrics Dashboard"));
    commands.spawn(DashboardWindow::new("Metrics Dashboard 2"));
}

fn update_metrics(time: Res<Time>) {
    let sec = time.delta_seconds_f64();
    let ms = 1000.0 * sec;
    histogram!("frame_time").record(ms);
    gauge!("frame_time").set(ms);
    counter!("num_frames").increment(1);
}
