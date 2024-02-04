//! A metrics dashboard for Bevy.
//!
//! This library enables Bevy apps to search and plot any metrics defined by
//! the [`metrics`](https://metrics.rs/) "ecosystem".
//!
//! # Getting Started
//!
//! See the "examples" directory.
//!
//! Steps for plotting your metrics:
//!   1. Define metrics using the [`metrics`] crate.
//!   2. Add the [`EguiPlugin`][egui_plugin] and [`DashboardPlugin`] to your app.
//!   3. Spawn an entity with the [`DashboardWindow`] component.
//!
//! [egui_plugin]: bevy_egui::EguiPlugin

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
