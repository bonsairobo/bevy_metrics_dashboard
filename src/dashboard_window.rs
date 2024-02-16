use crate::{
    namespace_tree::NamespaceTreeWindow,
    plots::{window_size_slider, MetricPlot, MetricPlotConfig},
    registry::{MetricKey, MetricsRegistry},
    search_bar::SearchBar,
};
use bevy::{prelude::*, utils::HashMap};
use bevy_egui::{
    egui::{self, Ui},
    EguiContexts,
};
use metrics::Unit;

#[derive(Clone, Event)]
pub struct RequestPlot {
    pub key: MetricKey,
    pub unit: Option<Unit>,
}

/// Cache of configs for plots that have been opened and removed.
#[derive(Default, Deref, DerefMut, Resource)]
pub struct CachedPlotConfigs(HashMap<MetricKey, MetricPlotConfig>);

/// An `egui` window that can search for metrics and plot them.
#[derive(Component)]
pub struct DashboardWindow {
    title: String,
    search_bar: SearchBar,
    plots: Vec<MetricPlot>,
    config: DashboardConfig,
}

#[derive(Default)]
pub struct DashboardConfig {
    pub global_window_size: Option<usize>,
    pub paused: bool,
}

impl DashboardWindow {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            search_bar: default(),
            plots: default(),
            config: default(),
        }
    }

    pub(crate) fn update_all(mut windows: Query<&mut Self>) {
        for mut window in &mut windows {
            if !window.config.paused {
                window.update();
            }
        }
    }

    pub(crate) fn update(&mut self) {
        for plot in &mut self.plots {
            plot.update();
        }
    }

    pub(crate) fn draw_all(
        mut commands: Commands,
        registry: Res<MetricsRegistry>,
        mut cached_configs: ResMut<CachedPlotConfigs>,
        mut ctxts: EguiContexts,
        mut requests: EventReader<RequestPlot>,
        mut windows: Query<(Entity, &mut Self)>,
    ) {
        let requests: Vec<_> = requests.read().cloned().collect();

        let ctxt = ctxts.ctx_mut();
        for (entity, mut window) in &mut windows {
            for RequestPlot { key, unit } in requests.iter().cloned() {
                window.add_plot(&registry, &cached_configs, key, unit);
            }

            let mut open = true;
            egui::Window::new(&window.title)
                .open(&mut open)
                .show(ctxt, |ui| {
                    ui.horizontal(|ui| {
                        window.add_search_results(&registry, &cached_configs, ui);
                        if ui.button("Browse").clicked() {
                            commands.spawn(NamespaceTreeWindow::new("Namespace Viewer"));
                        }
                    });
                    ui.collapsing("Global Settings", |ui| {
                        window.configure_ui(ui);
                    });
                    ui.separator();
                    window.draw_plots(&mut cached_configs, ui);
                });
            if !open {
                commands.entity(entity).despawn();
            }
        }
    }

    pub(crate) fn add_search_results(
        &mut self,
        registry: &MetricsRegistry,
        cached_configs: &CachedPlotConfigs,
        ui: &mut Ui,
    ) {
        let Some(selected) = self.search_bar.draw(registry, ui) else {
            return;
        };

        self.add_plot(
            registry,
            cached_configs,
            selected.key,
            selected.description.and_then(|d| d.unit),
        );
    }

    fn add_plot(
        &mut self,
        registry: &MetricsRegistry,
        cached_configs: &CachedPlotConfigs,
        key: MetricKey,
        unit: Option<Unit>,
    ) {
        // If we already have this metric, give it a unique name.
        let n_duplicates = self.plots.iter().filter(|p| p.key() == &key).count();

        let plot_config = cached_configs
            .get(&key)
            .cloned()
            .unwrap_or_else(|| MetricPlotConfig::default_for_kind(key.kind));
        self.plots.push(MetricPlot::new(
            registry,
            key.title(None, n_duplicates),
            key,
            unit,
            plot_config,
        ));
    }

    pub(crate) fn configure_ui(&mut self, ui: &mut Ui) {
        ui.checkbox(&mut self.config.paused, "Pause");

        let mut lock_window_size = self.config.global_window_size.is_some();
        ui.checkbox(&mut lock_window_size, "Link X Axes");
        if lock_window_size {
            let window_size = self.config.global_window_size.get_or_insert(500);
            ui.add(window_size_slider(window_size));
        } else {
            self.config.global_window_size = None;
        }
    }

    pub(crate) fn draw_plots(&mut self, cached_configs: &mut CachedPlotConfigs, ui: &mut Ui) {
        let mut remove_plots = Vec::new();

        egui::ScrollArea::vertical().show(ui, |ui| {
            for (i, plot) in self.plots.iter_mut().enumerate().rev() {
                // TODO: avoid string copy here?
                ui.collapsing(plot.name().to_owned(), |ui| {
                    if ui.button("Remove").clicked() {
                        remove_plots.push(i);
                    }

                    plot.draw(&self.config, ui);
                });
            }
        });

        for i in remove_plots {
            let plot = self.plots.remove(i);
            cached_configs.insert(plot.key().clone(), plot.clone_config());
        }
    }
}
