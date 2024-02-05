#![doc = include_str!("../README.md")]

mod core_metrics_plugin;
mod dashboard_plugin;
mod dashboard_window;
mod dropdown_list;
pub mod plots;
pub mod registry;
mod registry_plugin;
mod ring;
pub mod search_bar;

#[cfg(feature = "render_metrics")]
mod render_metrics_plugin;

pub use core_metrics_plugin::CoreMetricsPlugin;
pub use dashboard_plugin::DashboardPlugin;
pub use dashboard_window::{CachedPlotConfigs, DashboardWindow};
pub use registry_plugin::{ClearBucketsSystem, RegistryPlugin};

#[cfg(feature = "render_metrics")]
pub use render_metrics_plugin::RenderMetricsPlugin;

pub use bevy_egui;
pub use metrics;
pub use metrics_util;

// TODO:
// - per-window plot settings (e.g. window size)
// - don't assume there is a single egui context
// - add metadata section to plot that includes description and labels
// - basic math functions? (e.g. derivative)
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
