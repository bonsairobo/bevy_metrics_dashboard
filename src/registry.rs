use crate::metric_kind_str;
use bevy::{
    prelude::{default, Res, Resource},
    utils::HashMap,
};
use bevy_egui::egui::{text::LayoutJob, Color32, TextFormat};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use metrics::{Counter, Gauge, Histogram, Key, KeyName, Metadata, Recorder, SharedString, Unit};
use metrics_util::{
    registry::{AtomicStorage, Registry},
    MetricKind,
};
use std::sync::{Arc, RwLock};

#[derive(Clone, Resource)]
pub struct MetricsRegistry {
    inner: Arc<Inner>,
}

pub(crate) struct Inner {
    pub registry: Registry<Key, AtomicStorage>,
    pub descriptions: RwLock<HashMap<MetricKey, MetricDescription>>,
}

#[derive(Clone)]
pub struct MetricDescription {
    pub unit: Option<Unit>,
    pub text: SharedString,
}

impl Inner {
    fn new() -> Self {
        Self {
            registry: Registry::atomic(),
            descriptions: RwLock::new(Default::default()),
        }
    }
}

impl MetricsRegistry {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Inner::new()),
        }
    }

    pub fn inner_registry(&self) -> &Registry<Key, AtomicStorage> {
        &self.inner.registry
    }

    pub fn fuzzy_search_by_name(&self, input: &str) -> Vec<SearchResult> {
        let mut results = Vec::new();
        let matcher = SkimMatcherV2::default();
        let reg = self.inner_registry();
        let descriptions = self.inner.descriptions.read().unwrap();
        reg.visit_counters(|key, _| {
            if matcher.fuzzy_match(key.name(), input).is_some() {
                let key = MetricKey::new(key.clone(), MetricKind::Counter);
                let description = descriptions.get(&key).cloned();
                results.push(SearchResult { key, description });
            }
        });
        reg.visit_gauges(|key, _| {
            if matcher.fuzzy_match(key.name(), input).is_some() {
                let key = MetricKey::new(key.clone(), MetricKind::Gauge);
                let description = descriptions.get(&key).cloned();
                results.push(SearchResult { key, description });
            }
        });
        reg.visit_histograms(|key, _| {
            if matcher.fuzzy_match(key.name(), input).is_some() {
                let key = MetricKey::new(key.clone(), MetricKind::Histogram);
                let description = descriptions.get(&key).cloned();
                results.push(SearchResult { key, description });
            }
        });
        results
    }

    fn add_description_if_missing(&self, key: MetricKey, description: MetricDescription) {
        let mut descriptions = self.inner.descriptions.write().unwrap();
        descriptions.entry(key).or_insert(description);
    }
}

impl Default for MetricsRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MetricKey {
    pub key: Key,
    pub kind: MetricKind,
}

impl MetricKey {
    pub fn new(key: Key, kind: MetricKind) -> Self {
        Self { key, kind }
    }

    pub fn default_title(&self, n_duplicates: usize) -> String {
        if n_duplicates > 0 {
            format!(
                "{} ({}) {n_duplicates}",
                self.key.name(),
                metric_kind_str(self.kind)
            )
        } else {
            format!("{} ({})", self.key.name(), metric_kind_str(self.kind))
        }
    }
}

#[derive(Clone)]
pub struct SearchResult {
    pub key: MetricKey,
    pub description: Option<MetricDescription>,
}

impl SearchResult {
    pub fn dropdown_description(&self) -> LayoutJob {
        let mut job = LayoutJob::default();
        job.append(
            &self.key.default_title(0),
            0.0,
            TextFormat {
                // underline: Stroke::new(1.0, Color32::WHITE),
                color: Color32::WHITE,
                ..default()
            },
        );
        if let Some(description) = &self.description {
            job.append("\n", 0.0, default());
            job.append(
                &description.text,
                0.0,
                TextFormat {
                    color: Color32::GRAY,
                    italics: true,
                    ..default()
                },
            );
        }
        job
    }
}

impl Recorder for MetricsRegistry {
    fn describe_counter(&self, key_name: KeyName, unit: Option<Unit>, description: SharedString) {
        self.add_description_if_missing(
            MetricKey {
                key: Key::from_name(key_name),
                kind: MetricKind::Counter,
            },
            MetricDescription {
                unit,
                text: description,
            },
        );
    }

    fn describe_gauge(&self, key_name: KeyName, unit: Option<Unit>, description: SharedString) {
        self.add_description_if_missing(
            MetricKey {
                key: Key::from_name(key_name),
                kind: MetricKind::Gauge,
            },
            MetricDescription {
                unit,
                text: description,
            },
        );
    }

    fn describe_histogram(&self, key_name: KeyName, unit: Option<Unit>, description: SharedString) {
        self.add_description_if_missing(
            MetricKey {
                key: Key::from_name(key_name),
                kind: MetricKind::Histogram,
            },
            MetricDescription {
                unit,
                text: description,
            },
        );
    }

    fn register_counter(&self, key: &Key, _metadata: &Metadata<'_>) -> Counter {
        self.inner
            .registry
            .get_or_create_counter(key, |c| c.clone().into())
    }

    fn register_gauge(&self, key: &Key, _metadata: &Metadata<'_>) -> Gauge {
        self.inner
            .registry
            .get_or_create_gauge(key, |c| c.clone().into())
    }

    fn register_histogram(&self, key: &Key, _metadata: &Metadata<'_>) -> Histogram {
        self.inner
            .registry
            .get_or_create_histogram(key, |c| c.clone().into())
    }
}

pub fn clear_atomic_buckets(registry: Res<MetricsRegistry>) {
    let registry = registry.inner_registry();
    registry.visit_histograms(|_, h| {
        h.clear();
    });
}