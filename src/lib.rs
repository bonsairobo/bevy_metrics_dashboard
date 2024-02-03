pub mod dashboard_window;
pub mod finder;
pub mod plots;
pub mod registry;
mod ring;
mod search_bar;

pub use bevy_egui;
pub use metrics;
pub use metrics_util;

use bevy::prelude::*;
use dashboard_window::{CachedPlotConfigs, DashboardWindow};
use metrics::set_global_recorder;
use metrics_util::MetricKind;
use registry::{clear_atomic_buckets, MetricsRegistry};

// TODO:
// - automatic significant figures and units
// - automatically determine histogram bucket bounds
// - search by label
// - multiple metrics on one plot
// - rename and save plots
// - drag and drop plots (requires egui 0.26)

pub struct DashboardPlugin;

impl Plugin for DashboardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let registry = MetricsRegistry::default();
        set_global_recorder(registry.clone()).unwrap();
        app.init_resource::<CachedPlotConfigs>()
            .insert_resource(registry)
            .add_systems(
                Update,
                (DashboardWindow::update_all, DashboardWindow::draw_all),
            )
            .add_systems(Last, clear_atomic_buckets);
    }
}

fn metric_kind_str(kind: MetricKind) -> &'static str {
    match kind {
        MetricKind::Counter => "counter",
        MetricKind::Gauge => "gauge",
        MetricKind::Histogram => "histogram",
    }
}
