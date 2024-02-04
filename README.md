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

