use crate::registry::{MetricKey, MetricsRegistry};
use crate::ring::Ring;
use bevy::prelude::default;
use bevy_egui::egui::{Color32, Slider, TextEdit, Ui};
use egui_plot::{Bar, BarChart, Line, Plot, PlotPoints};
use float_ord::FloatOrd;
use metrics::atomics::AtomicU64;
use metrics_util::{AtomicBucket, MetricKind};
use smallvec::{smallvec, SmallVec};
use std::sync::{atomic::Ordering, Arc};

#[derive(Clone)]
pub enum MetricPlotConfig {
    Counter(CounterPlotConfig),
    Gauge(GaugePlotConfig),
    Histogram(HistogramPlotConfig),
}

impl MetricPlotConfig {
    pub fn default_for_kind(kind: MetricKind) -> Self {
        match kind {
            MetricKind::Counter => MetricPlotConfig::Counter(default()),
            MetricKind::Gauge => MetricPlotConfig::Gauge(default()),
            MetricKind::Histogram => MetricPlotConfig::Histogram(default()),
        }
    }
}

#[derive(Clone)]
pub struct CounterPlotConfig {
    pub window_size: usize,
}

impl Default for CounterPlotConfig {
    fn default() -> Self {
        Self { window_size: 500 }
    }
}

#[derive(Clone)]
pub struct GaugePlotConfig {
    pub smoothing_weight: f64,
    pub window_size: usize,
}

impl Default for GaugePlotConfig {
    fn default() -> Self {
        Self {
            smoothing_weight: 0.8,
            window_size: 500,
        }
    }
}

#[derive(Clone)]
pub struct HistogramPlotConfig {
    /// When `Some`, the bar chart is derived from a sliding window of
    /// data. Otherwise, the bar chart retains all data until it is reset or
    /// reconfigured.
    window_size: Option<usize>,
    /// Sorted list of boundaries between contiguous bucket ranges.
    bucket_bounds: SmallVec<[f64; 16]>,
    bucket_bounds_input: String,
}

impl Default for HistogramPlotConfig {
    fn default() -> Self {
        Self {
            window_size: Some(500),
            // Just initial values. These are configurable in the UI.
            bucket_bounds: smallvec![-10.0, 0.0, 10.0],
            bucket_bounds_input: default(),
        }
    }
}

pub struct MetricPlot {
    pub name: String,
    pub key: MetricKey,
    data: MetricPlotData,
}

#[allow(clippy::large_enum_variant)]
enum MetricPlotData {
    Counter(CounterData),
    Gauge(GaugeData),
    Histogram(HistogramData),
}

struct CounterData {
    source: Arc<AtomicU64>,
    ring: Ring<u64>,
    config: CounterPlotConfig,
}

impl CounterData {
    fn new(config: CounterPlotConfig, source: Arc<AtomicU64>) -> Self {
        let CounterPlotConfig { window_size } = config;
        Self {
            source,
            ring: Ring::new(window_size),
            config,
        }
    }

    fn configure_ui(&mut self, ui: &mut Ui) {
        if ui
            .add(Slider::new(&mut self.config.window_size, 100..=1000).text("Window Size"))
            .changed()
        {
            self.ring.resize(self.config.window_size);
        }
    }

    fn make_line(&self) -> Line {
        Line::new(PlotPoints::from_ys_f64(
            &self
                .ring
                .iter_chronological()
                .map(|&u| u as f64)
                .collect::<Vec<_>>(),
        ))
    }

    fn update(&mut self) {
        let value = self.source.load(Ordering::Relaxed);
        self.ring.push(value);
    }
}

struct GaugeData {
    source: Arc<AtomicU64>,
    smoother: Smoother,
    ring: Ring<f64>,
    config: GaugePlotConfig,
}

impl GaugeData {
    fn new(config: GaugePlotConfig, source: Arc<AtomicU64>) -> Self {
        let GaugePlotConfig {
            window_size,
            smoothing_weight,
        } = config;
        Self {
            source,
            smoother: Smoother::new(smoothing_weight),
            ring: Ring::new(window_size),
            config,
        }
    }

    fn configure_ui(&mut self, ui: &mut Ui) {
        ui.add(Slider::new(&mut self.config.smoothing_weight, 0.0..=1.0).text("Smoothing Weight"));
        self.smoother.weight = self.config.smoothing_weight;
        if ui
            .add(Slider::new(&mut self.config.window_size, 100..=1000).text("Window Size"))
            .changed()
        {
            self.ring.resize(self.config.window_size);
        }
    }

    fn make_line(&self) -> Line {
        Line::new(PlotPoints::from_ys_f64(
            &self.ring.iter_chronological().copied().collect::<Vec<_>>(),
        ))
    }

    fn update(&mut self) {
        let value = f64::from_bits(self.source.load(Ordering::Relaxed));
        self.smoother.add(value);
        self.ring.push(self.smoother.smoothed_value());
    }
}

struct HistogramData {
    source: Arc<AtomicBucket<f64>>,
    ring: Option<Ring<f64>>,
    bucket_counts: SmallVec<[u32; 16]>,

    config: HistogramPlotConfig,
}

impl HistogramData {
    fn new(config: HistogramPlotConfig, source: Arc<AtomicBucket<f64>>) -> Self {
        let n_buckets = config.bucket_bounds.len() + 1;

        Self {
            source,
            ring: None,
            bucket_counts: smallvec![0; n_buckets],
            config,
        }
    }

    fn configure_ui(&mut self, ui: &mut Ui) {
        let mut use_sliding_window = self.config.window_size.is_some();
        if ui
            .checkbox(&mut use_sliding_window, "Sliding Window")
            .changed()
            && !use_sliding_window
        {
            self.config.window_size = None;
            self.ring = None;
            self.bucket_counts.fill(0);
        }
        if use_sliding_window {
            let window_size = self.config.window_size.get_or_insert(500);
            if ui
                .add(Slider::new(window_size, 100..=1000).text("Window Size"))
                .changed()
            {
                self.ring = Some(Ring::new(*window_size));
            }
        }
        ui.horizontal(|ui| {
            ui.label("Bounds:");
            let response = TextEdit::singleline(&mut self.config.bucket_bounds_input)
                .hint_text("bucket bounds")
                .show(ui)
                .response;
            if response.changed() {
                self.update_bounds_from_input();
            }
        });
    }

    fn update_bounds_from_input(&mut self) {
        // Parse bounds as a whitespace-delimited list.
        let mut new_bounds = SmallVec::<[f64; 16]>::new();
        for token in self.config.bucket_bounds_input.split_whitespace() {
            let Ok(bound) = token.parse() else {
                return;
            };
            new_bounds.push(bound);
        }

        if new_bounds.is_empty() {
            return;
        }

        self.config.bucket_bounds = new_bounds;
        self.config
            .bucket_bounds
            .sort_unstable_by_key(|&b| FloatOrd(b));

        self.bucket_counts
            .resize(self.config.bucket_bounds.len() + 1, 0);
        self.bucket_counts.fill(0);
    }

    fn make_bar_chart(&self) -> BarChart {
        assert_eq!(
            self.bucket_counts.len(),
            self.config.bucket_bounds.len() + 1
        );

        let mut bars: Vec<_> = self
            .bucket_counts
            .iter()
            .map(|&c| Bar::new(0.0, c as f64))
            .collect();

        let mut avg_bar_width = 0.0;
        for (window_i, edges) in self.config.bucket_bounds.windows(2).enumerate() {
            let start = edges[0];
            let end = edges[1];
            let bar_i = window_i + 1;
            let width = end - start;
            let center = 0.5 * (start + end);
            let bar = &mut bars[bar_i];
            bar.argument = center;
            bar.bar_width = width;
            avg_bar_width += width;
        }
        avg_bar_width /= (self.config.bucket_bounds.len() - 1) as f64;

        let start = self.config.bucket_bounds[0];
        let end = *self.config.bucket_bounds.last().unwrap();

        let fst_bar = &mut bars[0];
        fst_bar.argument = start - 0.5 * avg_bar_width;
        fst_bar.bar_width = avg_bar_width;
        fst_bar.fill = Color32::BLUE;
        let last_bar = bars.last_mut().unwrap();
        last_bar.argument = end + 0.5 * avg_bar_width;
        last_bar.bar_width = avg_bar_width;
        last_bar.fill = Color32::BLUE;

        BarChart::new(bars)
    }

    fn update(&mut self) {
        if let Some(window_size) = self.config.window_size {
            // We are only counting within a sliding window, so clear
            // counts first.
            self.bucket_counts.fill(0);
            // We only need to take the latest values up to the ring's
            // capacity. The histogram doesn't actually care about the order
            // of elements, it just needs to always have the most recent
            // N elements.
            let ring = self.ring.get_or_insert_with(|| Ring::new(window_size));
            let mut taken = 0;
            self.source.data_with(|block| {
                let mut block_iter = block.iter().rev().copied();
                while taken < ring.size() {
                    if let Some(value) = block_iter.next() {
                        ring.push(value);
                        taken += 1;
                    } else {
                        break;
                    }
                }
            });
            for &value in ring.iter_chronological() {
                add_value_to_bucket(&self.config.bucket_bounds, value, &mut self.bucket_counts);
            }
        } else {
            // Keep adding to the existing buckets.
            self.source.data_with(|block| {
                for &value in block {
                    add_value_to_bucket(&self.config.bucket_bounds, value, &mut self.bucket_counts);
                }
            });
        }
    }
}

impl MetricPlot {
    pub(crate) fn new(
        registry: &MetricsRegistry,
        name: impl Into<String>,
        key: MetricKey,
        config: MetricPlotConfig,
    ) -> Self {
        let data = match config {
            MetricPlotConfig::Counter(config) => {
                let source = registry
                    .inner_registry()
                    .get_or_create_counter(&key.key, Arc::clone);
                MetricPlotData::Counter(CounterData::new(config, source))
            }
            MetricPlotConfig::Gauge(config) => {
                let source = registry
                    .inner_registry()
                    .get_or_create_gauge(&key.key, Arc::clone);
                MetricPlotData::Gauge(GaugeData::new(config, source))
            }
            MetricPlotConfig::Histogram(bar_config) => {
                let source = registry
                    .inner_registry()
                    .get_or_create_histogram(&key.key, Arc::clone);
                MetricPlotData::Histogram(HistogramData::new(bar_config, source))
            }
        };

        Self {
            name: name.into(),
            key,
            data,
        }
    }

    pub fn clone_config(&self) -> MetricPlotConfig {
        match &self.data {
            MetricPlotData::Counter(data) => MetricPlotConfig::Counter(data.config.clone()),
            MetricPlotData::Gauge(data) => MetricPlotConfig::Gauge(data.config.clone()),
            MetricPlotData::Histogram(data) => MetricPlotConfig::Histogram(data.config.clone()),
        }
    }

    pub fn update(&mut self) {
        match &mut self.data {
            MetricPlotData::Counter(data) => {
                data.update();
            }
            MetricPlotData::Gauge(data) => {
                data.update();
            }
            MetricPlotData::Histogram(data) => {
                data.update();
            }
        }
    }

    pub fn draw(&mut self, ui: &mut Ui) -> Option<PlotAction> {
        let mut action = None;

        let Self { name, data, .. } = self;

        ui.collapsing(&*name, |ui| {
            if ui.button("Remove").clicked() {
                action = Some(PlotAction::Remove);
            }
            draw_plot(name, data, ui);
        });

        action
    }
}

fn add_value_to_bucket(bucket_bounds: &[f64], value: f64, bucket_counts: &mut [u32]) {
    let bucket_i = match bucket_bounds.binary_search_by_key(&FloatOrd(value), |&d| FloatOrd(d)) {
        Ok(i) => i,
        Err(i) => i,
    };
    bucket_counts[bucket_i] += 1;
}

fn draw_plot(name: &str, data: &mut MetricPlotData, ui: &mut Ui) {
    let new_plot = || Plot::new(name).view_aspect(2.0);

    match data {
        MetricPlotData::Counter(data) => {
            ui.collapsing("Settings", |ui| {
                data.configure_ui(ui);
            });

            let latest = data.ring.latest();
            ui.label(format!("latest = {latest:.3}"));

            let line = data.make_line();
            new_plot()
                .x_axis_label("frame")
                .show(ui, |plot_ui| plot_ui.line(line));
        }
        MetricPlotData::Gauge(data) => {
            ui.collapsing("Settings", |ui| {
                data.configure_ui(ui);
            });

            let latest = data.ring.latest();
            ui.label(format!("latest = {latest:.3}"));

            let line = data.make_line();
            new_plot()
                .x_axis_label("frame")
                .show(ui, |plot_ui| plot_ui.line(line));
        }
        MetricPlotData::Histogram(data) => {
            ui.collapsing("Settings", |ui| {
                data.configure_ui(ui);
            });

            let chart = data.make_bar_chart();
            new_plot()
                .y_axis_label("count")
                .show(ui, |plot_ui| plot_ui.bar_chart(chart));
        }
    }
}

pub enum PlotAction {
    Remove,
}

struct Smoother {
    smoothed_value: Option<f64>,
    weight: f64,
}

impl Smoother {
    fn new(weight: f64) -> Self {
        Self {
            smoothed_value: None,
            weight: weight.clamp(0.0, 1.0),
        }
    }

    fn add(&mut self, value: f64) {
        self.smoothed_value =
            Some((1.0 - self.weight) * value + self.weight * self.smoothed_value.unwrap_or(value));
    }

    fn smoothed_value(&self) -> f64 {
        self.smoothed_value.unwrap()
    }
}
