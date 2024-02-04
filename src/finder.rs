use crate::{
    dropdown_list::dropdown_list,
    registry::{MetricsRegistry, SearchResult},
};
use bevy::tasks::{block_on, AsyncComputeTaskPool, Task};
use bevy_egui::egui::{TextEdit, Ui};
use std::time::{Duration, Instant};

pub struct MetricsFinder {
    search_input: String,
    input_dirty: bool,
    last_search_time: Instant,
    search_task: Option<Task<Vec<SearchResult>>>,
    search_results: Vec<SearchResult>,
}

impl Default for MetricsFinder {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricsFinder {
    pub fn new() -> Self {
        Self {
            search_input: Default::default(),
            input_dirty: true,
            last_search_time: Instant::now(),
            search_task: Default::default(),
            search_results: Default::default(),
        }
    }

    pub fn draw(&mut self, registry: &MetricsRegistry, ui: &mut Ui) -> Option<SearchResult> {
        // Draw search box.
        let maybe_selected = ui
            .horizontal(|ui| {
                ui.label("Search:");
                let response = TextEdit::singleline(&mut self.search_input)
                    .hint_text("metric name")
                    .show(ui)
                    .response;
                if response.changed() {
                    self.input_dirty = true;
                }
                dropdown_list(
                    response,
                    ui,
                    "metric-search-dropdown",
                    self.search_results.iter(),
                    |&s| s.dropdown_description(),
                )
                .cloned()
            })
            .inner;

        // Check if we have new search results.
        if let Some(task) = self.search_task.take() {
            if task.is_finished() {
                self.search_results = block_on(task);
            } else {
                self.search_task = Some(task);
            }
        }

        if self.input_dirty && self.last_search_time.elapsed() > Duration::from_millis(250) {
            // Spawn task to search the registry, just to avoid long frame times
            // when searching a large registry.
            self.last_search_time = Instant::now();
            let search_input = self.search_input.clone();
            let task_registry = registry.clone();
            let task = AsyncComputeTaskPool::get()
                .spawn(async move { task_registry.fuzzy_search_by_name(&search_input) });
            self.search_task = Some(task);
            self.input_dirty = false;
        }

        maybe_selected
    }
}
