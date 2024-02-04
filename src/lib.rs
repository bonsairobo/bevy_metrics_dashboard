#![doc = include_str!("../README.md")]

mod dashboard_window;
mod finder;
mod plots;
mod plugin;
mod registry;
mod ring;
mod search_bar;

pub use dashboard_window::DashboardWindow;
pub use plugin::DashboardPlugin;
pub use registry::MetricsRegistry;

pub use bevy_egui;
pub use metrics;
pub use metrics_util;

// TODO:
// - low-level API for integrating finder + plots into a custom UI
// - closing dashboard windows
// - automatic significant figures
// - automatically determine histogram bucket bounds
// - search by label and description
// - multiple metrics on one plot
// - rename and save plots
// - drag and drop plots (requires egui 0.26)

use metrics_util::MetricKind;

fn metric_kind_str(kind: MetricKind) -> &'static str {
    match kind {
        MetricKind::Counter => "counter",
        MetricKind::Gauge => "gauge",
        MetricKind::Histogram => "histogram",
    }
}
