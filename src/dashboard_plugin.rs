use crate::{dashboard_window::CachedPlotConfigs, ClearBucketsSystem, DashboardWindow};
use bevy::prelude::*;

/// Updates and renders all [`DashboardWindow`] entities.
pub struct DashboardPlugin;

impl Plugin for DashboardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<CachedPlotConfigs>()
            .add_systems(Update, DashboardWindow::draw_all)
            // Enforce strict ordering:
            // metrics producers (before Last) --> metrics consumers --> bucket clearing
            .add_systems(Last, DashboardWindow::update_all.before(ClearBucketsSystem));
    }
}
