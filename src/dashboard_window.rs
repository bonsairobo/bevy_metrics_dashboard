use crate::{
    finder::MetricsFinder,
    plots::{MetricPlot, MetricPlotConfig, PlotAction},
    registry::{MetricKey, MetricsRegistry},
};
use bevy::{prelude::*, utils::HashMap};
use bevy_egui::{
    egui::{self, Ui},
    EguiContexts,
};

#[derive(Default, Deref, DerefMut, Resource)]
pub struct CachedPlotConfigs(HashMap<MetricKey, MetricPlotConfig>);

#[derive(Component)]
pub struct DashboardWindow {
    title: String,
    finder: MetricsFinder,
    plots: Vec<MetricPlot>,
}

impl DashboardWindow {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            finder: default(),
            plots: default(),
        }
    }

    pub fn update_all(mut windows: Query<&mut DashboardWindow>) {
        for mut window in &mut windows {
            window.update();
        }
    }

    pub fn update(&mut self) {
        for plot in &mut self.plots {
            plot.update();
        }
    }

    pub fn draw_all(
        registry: Res<MetricsRegistry>,
        mut cached_configs: ResMut<CachedPlotConfigs>,
        mut ctxts: EguiContexts,
        mut windows: Query<&mut DashboardWindow>,
    ) {
        let ctxt = ctxts.ctx_mut();
        for mut window in &mut windows {
            egui::Window::new(&window.title).show(ctxt, |ui| {
                if let Some(selected) = window.finder.draw(&registry, ui) {
                    // If we already have this metric, give it a unique name.
                    let n_duplicates = window
                        .plots
                        .iter()
                        .filter(|p| p.key == selected.key)
                        .count();

                    let plot_config = cached_configs
                        .get(&selected.key)
                        .cloned()
                        .unwrap_or_else(|| MetricPlotConfig::default_for_kind(selected.key.kind));
                    window.plots.push(MetricPlot::new(
                        &registry,
                        selected.key.default_title(n_duplicates),
                        selected.key,
                        selected.description.and_then(|d| d.unit),
                        plot_config,
                    ));
                }
                window.draw_plots(&mut cached_configs, ui);
            });
        }
    }

    pub fn draw_plots(&mut self, cached_configs: &mut CachedPlotConfigs, ui: &mut Ui) {
        let mut remove_plots = Vec::new();

        egui::ScrollArea::vertical().show(ui, |ui| {
            for (i, plot) in self.plots.iter_mut().enumerate().rev() {
                let action = plot.draw(ui);

                #[allow(clippy::single_match)]
                match action {
                    Some(PlotAction::Remove) => {
                        remove_plots.push(i);
                    }
                    None => (),
                }
            }
        });

        for i in remove_plots {
            let plot = self.plots.remove(i);
            cached_configs.insert(plot.key.clone(), plot.clone_config());
        }
    }
}
