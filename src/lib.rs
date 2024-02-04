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
// - automatic significant figures
// - automatically determine histogram bucket bounds
// - search by label and description
// - multiple metrics on one plot
// - rename and save plots
// - drag and drop plots (requires egui 0.26)

#[derive(Default)]
pub struct DashboardPlugin {
    registry: Option<MetricsRegistry>,
}

impl DashboardPlugin {
    pub fn new() -> Self {
        Self::default()
    }

    /// Provide your own instance of a registry.
    ///
    /// WARNING: Using this constructor will silence errors if it fails to
    /// call [`set_global_recorder`], assuming that the user already did this
    /// manually with `registry`.
    pub fn with_registry(registry: MetricsRegistry) -> Self {
        Self {
            registry: Some(registry),
        }
    }
}

impl Plugin for DashboardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let app = app.init_resource::<CachedPlotConfigs>();
        let registry = if let Some(registry) = &self.registry {
            _ = set_global_recorder(registry.clone());
            registry.clone()
        } else {
            let registry = MetricsRegistry::default();
            if let Err(e) = set_global_recorder(registry.clone()) {
                error!("Failed to set global recorder: {e}");
            }
            registry
        };
        app.insert_resource(registry)
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
