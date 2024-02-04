use crate::{
    dashboard_window::CachedPlotConfigs, registry::clear_atomic_buckets, DashboardWindow,
    MetricsRegistry,
};
use bevy::prelude::*;
use metrics::set_global_recorder;

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
