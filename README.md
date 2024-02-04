A metrics dashboard for Bevy.

This library enables Bevy apps to search and plot any metrics defined by
the [`metrics`](https://metrics.rs/) "ecosystem".

![screen](https://raw.githubusercontent.com/bonsairobo/bevy_metrics_dashboard/main/images/screen.png)

# Should I use this?

1. Do you find custom metrics useful when debugging?
1. Do you like the `metrics` ecosystem?
1. Do you want a cheap way to observe your metrics directly within your Bevy app?

# Getting Started

See the "examples" directory.

Steps for plotting your metrics:

  1. Define metrics using the [`metrics`] crate.
  1. Add the [`EguiPlugin`][egui_plugin] and [`DashboardPlugin`] to your app.
  1. Spawn an entity with the [`DashboardWindow`] component.

[egui_plugin]: bevy_egui::EguiPlugin

# How does it work?

The `metrics` crate lets developers define metrics in their code using simple
macros. Each process has a global registry (AKA "recorder") of all of the
metrics that have been used or described in code. Many existing metrics
"exporters" will use their registry to send metrics to a monitoring service
like Prometheus or CloudWatch.

In contrast, `bevy_metrics_dashboard` does not depend on any external service;
all metrics can be plotted directly in the application where they are defined.
The provided Bevy plugin defines and installs a registry, and the dashboard
widget lets users search for metrics and plot them.
