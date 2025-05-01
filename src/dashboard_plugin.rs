use crate::{
    dashboard_window::{CachedPlotConfigs, RequestPlot},
    namespace_tree::NamespaceTreeWindow,
    ClearBucketsSystem, DashboardWindow,
};
use bevy::prelude::*;
use bevy_egui::EguiContextPass;

/// Updates and renders all [`DashboardWindow`] and [`NamespaceTreeWindow`]
/// entities.
pub struct DashboardPlugin;

impl Plugin for DashboardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<RequestPlot>()
            .init_resource::<CachedPlotConfigs>()
            .add_systems(
                EguiContextPass,
                (DashboardWindow::draw_all, NamespaceTreeWindow::draw_all),
            )
            // Enforce strict ordering:
            // metrics producers (before Last) --> metrics consumers --> bucket clearing
            .add_systems(
                Last,
                DashboardWindow::update_plots_on_all_windows.before(ClearBucketsSystem),
            );
    }
}
