#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

mod core_metrics_plugin;
#[cfg(feature = "bevy_egui")]
mod dashboard_plugin;
mod dashboard_window;
mod dropdown_list;
mod namespace_tree;
pub mod plots;
pub mod registry;
mod registry_plugin;
mod ring;
mod search_bar;

#[cfg(feature = "render_metrics")]
mod render_metrics_plugin;

pub use core_metrics_plugin::CoreMetricsPlugin;
#[cfg(feature = "bevy_egui")]
pub use dashboard_plugin::DashboardPlugin;
pub use dashboard_window::{CachedPlotConfigs, DashboardConfig, DashboardWindow, RequestPlot};
pub use namespace_tree::NamespaceTreeWindow;
pub use registry_plugin::{ClearBucketsSystem, RegistryPlugin};
pub use search_bar::SearchBar;

#[cfg(feature = "render_metrics")]
pub use render_metrics_plugin::RenderMetricsPlugin;

pub use egui;
pub use egui_plot;

pub use metrics;
pub use metrics_util;

// TODO:
// - dedup some of the code between gauge and counter widgets
// - extract egui-only crate (with no bevy dep)
// - don't assume there is a single egui context
// - automatic significant figures
// - automatically determine histogram bucket bounds
// - search by label and description
// - multiple metrics on one plot
// - rename and save plots
// - drag and drop plots (requires egui 0.26)

use metrics::Unit;
use metrics_util::MetricKind;

fn metric_kind_str(kind: MetricKind) -> &'static str {
    match kind {
        MetricKind::Counter => "counter",
        MetricKind::Gauge => "gauge",
        MetricKind::Histogram => "histogram",
    }
}

fn unit_str(unit: Unit) -> &'static str {
    match unit {
        Unit::Count => "count",
        Unit::Percent => "%",
        Unit::Seconds => "s",
        Unit::Milliseconds => "ms",
        Unit::Microseconds => "μs",
        Unit::Nanoseconds => "ns",
        Unit::Tebibytes => "TiB",
        Unit::Gibibytes => "GiB",
        Unit::Mebibytes => "MiB",
        Unit::Kibibytes => "KiB",
        Unit::Bytes => "B",
        Unit::TerabitsPerSecond => "Tb/s",
        Unit::GigabitsPerSecond => "Gb/s",
        Unit::MegabitsPerSecond => "Mb/s",
        Unit::KilobitsPerSecond => "Kb/s",
        Unit::BitsPerSecond => "b/s",
        Unit::CountPerSecond => "hz",
    }
}
