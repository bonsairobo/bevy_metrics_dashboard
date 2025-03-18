//! Widgets for plotting metrics.

use crate::dashboard_window::DashboardConfig;
use crate::registry::{MetricKey, MetricsRegistry};
use crate::ring::Ring;
use crate::unit_str;
use bevy::prelude::default;
use egui::{Color32, DragValue, Slider, Ui};
use egui_plot::{Bar, BarChart, Line, Plot, PlotPoint, PlotPoints};
use float_ord::FloatOrd;
use metrics::atomics::AtomicU64;
use metrics::Unit;
use metrics_util::{AtomicBucket, MetricKind};
use smallvec::{smallvec, SmallVec};
use std::sync::{atomic::Ordering, Arc};

// BUG: If you have a constant metric, it results in a perfectly horizontal line
// plot. Such a plot seems invisible because the axes auto-fit with a height
// of zero.
//
// https://github.com/emilk/egui/issues/3970

/// Configuration for one [`MetricPlot`].
#[allow(missing_docs)]
#[derive(Clone)]
pub enum MetricPlotConfig {
    Counter(CounterPlotConfig),
    Gauge(GaugePlotConfig),
    Histogram(HistogramPlotConfig),
}

impl MetricPlotConfig {
    /// Default configuration for the given `kind` of metric being plotted.
    pub fn default_for_kind(kind: MetricKind) -> Self {
        match kind {
            MetricKind::Counter => MetricPlotConfig::Counter(default()),
            MetricKind::Gauge => MetricPlotConfig::Gauge(default()),
            MetricKind::Histogram => MetricPlotConfig::Histogram(default()),
        }
    }
}

/// Configuration for a [`MetricPlot`] of [`MetricKind::Counter`].
#[derive(Clone)]
pub struct CounterPlotConfig {
    /// How many samples are drawn in one plot.
    pub window_size: usize,
    /// If true, plots the time derivative.
    pub derivative: bool,
}

impl Default for CounterPlotConfig {
    fn default() -> Self {
        Self {
            window_size: 500,
            derivative: false,
        }
    }
}

/// Configuration for a [`MetricPlot`] of [`MetricKind::Gauge`].
#[derive(Clone)]
pub struct GaugePlotConfig {
    /// A weight in `0.0..=1.0` used for exponential smoothing.
    pub smoothing_weight: f64,
    /// How many samples are drawn in one plot.
    pub window_size: usize,
    /// If true, plots the time derivative.
    pub derivative: bool,
}

impl Default for GaugePlotConfig {
    fn default() -> Self {
        Self {
            smoothing_weight: 0.8,
            window_size: 500,
            derivative: false,
        }
    }
}

/// Configuration for a [`MetricPlot`] of [`MetricKind::Histogram`].
#[derive(Clone)]
pub struct HistogramPlotConfig {
    /// When `Some`, the bar chart is derived from a sliding window of
    /// data. Otherwise, the bar chart retains all data until it is reset or
    /// reconfigured.
    pub window_size: Option<usize>,
    #[allow(missing_docs)]
    pub buckets: BucketConfig,
}

impl Default for HistogramPlotConfig {
    fn default() -> Self {
        Self {
            window_size: Some(500),
            buckets: default(),
        }
    }
}

/// Configuration of the buckets in a histogram.
#[derive(Clone)]
pub struct BucketConfig {
    /// Sorted list of boundaries between contiguous bucket ranges.
    ///
    /// Derived from [`Self::range_input`].
    pub bounds: BoundsVec,
    #[allow(missing_docs)]
    pub range_input: BucketRange,
}

#[allow(missing_docs)]
pub type BoundsVec = SmallVec<[f64; 16]>;
#[allow(missing_docs)]
pub type CountsVec = SmallVec<[u32; 16]>;

/// A uniformly distributed set of buckets.
#[allow(missing_docs)]
#[derive(Clone)]
pub struct BucketRange {
    pub n_buckets: usize,
    pub min: f64,
    pub max: f64,
}

impl BucketRange {
    /// Prevent `self.min >= self.max` by clamping `self.min`.
    pub fn clamp_min(&mut self) {
        self.min = self.min.min(self.max - 0.001);
    }

    /// Prevent `self.max <= self.min` by clamping `self.max`.
    pub fn clamp_max(&mut self) {
        self.max = (self.min + 0.001).max(self.max);
    }

    /// Calculate bounds of all buckets.
    pub fn get_bounds(&self) -> BoundsVec {
        assert!(self.max > self.min, "{} > {}", self.max, self.min);
        let width = (self.max - self.min) / self.n_buckets as f64;
        (0..=self.n_buckets)
            .map(|i| self.min + i as f64 * width)
            .collect()
    }
}

impl Default for BucketRange {
    fn default() -> Self {
        Self {
            min: 0.0,
            max: 10.0,
            n_buckets: 10,
        }
    }
}

impl BucketConfig {
    /// Calculate bounds of all buckets.
    ///
    /// Returns `None` if there are zero buckets.
    pub fn get_bounds(&self) -> Option<BoundsVec> {
        if self.range_input.n_buckets == 0 {
            return None;
        }
        let mut new_bounds = self.range_input.get_bounds();
        new_bounds.sort_unstable_by_key(|&b| FloatOrd(b));
        Some(new_bounds)
    }
}

impl Default for BucketConfig {
    fn default() -> Self {
        let range_input = BucketRange::default();
        let bounds = range_input.get_bounds();
        Self {
            bounds,
            range_input,
        }
    }
}

/// A plot for any [`MetricKind`], rendering with [`egui_plot`].
pub struct MetricPlot {
    name: String,
    key: MetricKey,
    unit: Option<Unit>,
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
        let CounterPlotConfig { window_size, .. } = config;
        Self {
            source,
            ring: Ring::new(window_size),
            config,
        }
    }

    fn handle_global_config(&mut self, dash_config: &DashboardConfig) {
        if let Some(window_size) = dash_config.global_window_size {
            self.config.window_size = window_size;
            self.ring.set_max_len(self.config.window_size);
        }
    }

    fn configure_ui(&mut self, enable_window_size: bool, ui: &mut Ui) {
        ui.checkbox(&mut self.config.derivative, "Derivative");

        if enable_window_size
            && ui
                .add(window_size_slider(&mut self.config.window_size))
                .changed()
        {
            self.ring.set_max_len(self.config.window_size);
        }
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
            ..
        } = config;
        Self {
            source,
            smoother: Smoother::new(smoothing_weight),
            ring: Ring::new(window_size),
            config,
        }
    }

    fn handle_global_config(&mut self, dash_config: &DashboardConfig) {
        if let Some(window_size) = dash_config.global_window_size {
            self.config.window_size = window_size;
            self.ring.set_max_len(self.config.window_size);
        }
    }

    fn configure_ui(&mut self, enable_window_size: bool, ui: &mut Ui) {
        ui.checkbox(&mut self.config.derivative, "Derivative");

        if enable_window_size
            && ui
                .add(window_size_slider(&mut self.config.window_size))
                .changed()
        {
            self.ring.set_max_len(self.config.window_size);
        }

        ui.add(Slider::new(&mut self.config.smoothing_weight, 0.0..=1.0).text("Smoothing Weight"));
        self.smoother.weight = self.config.smoothing_weight;
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
    bucket_counts: CountsVec,
    config: HistogramPlotConfig,
}

impl HistogramData {
    fn new(config: HistogramPlotConfig, source: Arc<AtomicBucket<f64>>) -> Self {
        let n_buckets = config.buckets.bounds.len() + 1;

        Self {
            source,
            ring: None,
            bucket_counts: smallvec![0; n_buckets],
            config,
        }
    }

    fn configure_ui(&mut self, ui: &mut Ui) {
        let mut update = false;
        ui.horizontal(|ui| {
            update |= ui
                .add(
                    DragValue::new(&mut self.config.buckets.range_input.n_buckets)
                        .prefix("Buckets: ")
                        .speed(0.1),
                )
                .changed();
            update |= ui
                .add(
                    DragValue::new(&mut self.config.buckets.range_input.min)
                        .prefix("Min: ")
                        .speed(0.1),
                )
                .changed();
            self.config.buckets.range_input.clamp_max();
            update |= ui
                .add(
                    DragValue::new(&mut self.config.buckets.range_input.max)
                        .prefix("Max: ")
                        .speed(0.1),
                )
                .changed();
            self.config.buckets.range_input.clamp_min();
        });
        if update {
            self.update_bounds_from_input();
        }

        ui.separator();

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
            if ui.add(window_size_slider(window_size)).changed() {
                self.ring = Some(Ring::new(*window_size));
            }
        }
    }

    fn update_bounds_from_input(&mut self) {
        let Some(new_bounds) = self.config.buckets.get_bounds() else {
            return;
        };

        self.config.buckets.bounds = new_bounds;

        self.bucket_counts
            .resize(self.config.buckets.bounds.len() + 1, 0);
        self.bucket_counts.fill(0);
    }

    fn make_bar_chart(&self) -> BarChart {
        assert_eq!(
            self.bucket_counts.len(),
            self.config.buckets.bounds.len() + 1
        );

        let mut bars: Vec<_> = self
            .bucket_counts
            .iter()
            .map(|&c| Bar::new(0.0, c as f64))
            .collect();

        let mut avg_bar_width = 0.0;
        for (window_i, edges) in self.config.buckets.bounds.windows(2).enumerate() {
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
        avg_bar_width /= (self.config.buckets.bounds.len() - 1) as f64;

        let start = self.config.buckets.bounds[0];
        let end = *self.config.buckets.bounds.last().unwrap();

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
                while taken < ring.max_len() {
                    if let Some(value) = block_iter.next() {
                        ring.push(value);
                        taken += 1;
                    } else {
                        break;
                    }
                }
            });
            for &value in ring.iter_chronological() {
                add_value_to_bucket(&self.config.buckets.bounds, value, &mut self.bucket_counts);
            }
        } else {
            // Keep adding to the existing buckets.
            self.source.data_with(|block| {
                for &value in block {
                    add_value_to_bucket(
                        &self.config.buckets.bounds,
                        value,
                        &mut self.bucket_counts,
                    );
                }
            });
        }
    }
}

impl MetricPlot {
    /// Create a new plot from the metric identified by `key`.
    ///
    /// The metric will be found in `registry`. If it does not exist, it will
    /// be created.
    pub fn new(
        registry: &MetricsRegistry,
        name: impl Into<String>,
        key: MetricKey,
        unit: Option<Unit>,
        config: MetricPlotConfig,
    ) -> Self {
        let data = match config {
            MetricPlotConfig::Counter(config) => {
                let source = registry.get_or_create_counter(&key.key);
                MetricPlotData::Counter(CounterData::new(config, source))
            }
            MetricPlotConfig::Gauge(config) => {
                let source = registry.get_or_create_gauge(&key.key);
                MetricPlotData::Gauge(GaugeData::new(config, source))
            }
            MetricPlotConfig::Histogram(bar_config) => {
                let source = registry.get_or_create_histogram(&key.key);
                MetricPlotData::Histogram(HistogramData::new(bar_config, source))
            }
        };

        Self {
            name: name.into(),
            key,
            unit,
            data,
        }
    }

    /// Name of the plot.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The key of the metric being plotted.
    pub fn key(&self) -> &MetricKey {
        &self.key
    }

    /// Clone this plot's configuration.
    pub fn clone_config(&self) -> MetricPlotConfig {
        match &self.data {
            MetricPlotData::Counter(data) => MetricPlotConfig::Counter(data.config.clone()),
            MetricPlotData::Gauge(data) => MetricPlotConfig::Gauge(data.config.clone()),
            MetricPlotData::Histogram(data) => MetricPlotConfig::Histogram(data.config.clone()),
        }
    }

    /// Pull metric data from the source.
    ///
    /// This should run in the [`Last`](bevy::prelude::Last) schedule **before**
    /// [`ClearBucketsSystem`](crate::ClearBucketsSystem) to ensure no data
    /// is missed.
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

    /// Draw the plot using `ui`.
    pub fn draw(&mut self, dash_config: &DashboardConfig, ui: &mut Ui) {
        let Self {
            name, unit, data, ..
        } = self;

        draw_plot(dash_config, name, *unit, data, ui);
    }
}

fn add_value_to_bucket(bucket_bounds: &[f64], value: f64, bucket_counts: &mut [u32]) {
    let bucket_i = match bucket_bounds.binary_search_by_key(&FloatOrd(value), |&d| FloatOrd(d)) {
        Ok(i) => i,
        Err(i) => i,
    };
    bucket_counts[bucket_i] += 1;
}

fn draw_plot(
    dash_config: &DashboardConfig,
    name: &str,
    unit: Option<Unit>,
    data: &mut MetricPlotData,
    ui: &mut Ui,
) {
    let new_plot = || {
        Plot::new(name)
            .allow_scroll(false)
            .view_aspect(2.0)
            .auto_bounds([true, true].into())
    };

    match data {
        MetricPlotData::Counter(data) => {
            if let Some(latest) = data.ring.latest() {
                ui.label(format!("latest = {latest:.3}"));
            }

            let mut plot_points = data.ring.make_plot_points();
            if data.config.derivative {
                derivative(&mut plot_points);
            }
            let line = Line::new(PlotPoints::Owned(plot_points));
            let mut plot = new_plot().x_axis_label("frame");
            if let Some(unit) = unit {
                plot = plot.y_axis_label(unit_str(unit));
            }
            plot.show(ui, |plot_ui| plot_ui.line(line));

            data.handle_global_config(dash_config);
            ui.collapsing("Settings", |ui| {
                data.configure_ui(dash_config.global_window_size.is_none(), ui);
            });
        }
        MetricPlotData::Gauge(data) => {
            if let Some(latest) = data.ring.latest() {
                ui.label(format!("latest = {latest:.3}"));
            }

            let mut plot_points = data.ring.make_plot_points();
            if data.config.derivative {
                derivative(&mut plot_points);
            }
            let line = Line::new(PlotPoints::Owned(plot_points));
            let mut plot = new_plot().x_axis_label("frame");
            if let Some(unit) = unit {
                plot = plot.y_axis_label(unit_str(unit));
            }
            plot.show(ui, |plot_ui| plot_ui.line(line));

            data.handle_global_config(dash_config);
            ui.collapsing("Settings", |ui| {
                data.configure_ui(dash_config.global_window_size.is_none(), ui);
            });
        }
        MetricPlotData::Histogram(data) => {
            let chart = data.make_bar_chart();
            let mut plot = new_plot().y_axis_label("count");
            if let Some(unit) = unit {
                plot = plot.x_axis_label(unit_str(unit));
            }
            plot.show(ui, |plot_ui| plot_ui.bar_chart(chart));

            ui.collapsing("Settings", |ui| {
                data.configure_ui(ui);
            });
        }
    }
}

pub(crate) fn window_size_slider(size: &mut usize) -> Slider {
    Slider::new(size, 100..=5000).text("Window Size")
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

fn derivative(points: &mut Vec<PlotPoint>) {
    if points.is_empty() {
        return;
    }

    if points.len() > 1 {
        for i in 0..points.len() - 1 {
            let dy = points[i + 1].y - points[i].y;
            let dx = points[i + 1].x - points[i].x;
            points[i].y = dy / dx;
        }
    }
    points.pop();
}
