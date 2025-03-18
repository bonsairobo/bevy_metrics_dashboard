use crate::egui_plot::PlotPoint;
use std::collections::VecDeque;

/// A resizable ring buffer.
pub struct Ring<T> {
    elements: VecDeque<T>,
    max_len: usize,
}

impl<T: Clone + Default> Ring<T> {
    pub fn new(max_len: usize) -> Self {
        Self {
            elements: VecDeque::with_capacity(max_len),
            max_len,
        }
    }

    pub fn max_len(&self) -> usize {
        self.max_len
    }

    pub fn set_max_len(&mut self, new_max_len: usize) {
        self.max_len = new_max_len;

        if self.max_len > self.elements.capacity() {
            let add_cap = self.max_len - self.elements.capacity();
            self.elements.reserve(add_cap);
        }
        if self.max_len < self.elements.len() {
            self.elements.truncate(self.max_len);
        }
    }

    pub fn latest(&self) -> Option<&T> {
        self.elements.front()
    }

    /// Overwrites the oldest element if capacity limit is reached.
    pub fn push(&mut self, elem: T) {
        while self.elements.len() >= self.max_len {
            self.elements.pop_back();
        }
        self.elements.push_front(elem);
    }

    pub fn iter_chronological(&self) -> impl Iterator<Item = &T> {
        self.elements.iter().rev()
    }

    pub fn make_plot_points(&self) -> Vec<PlotPoint>
    where
        T: num_traits::NumCast,
    {
        (0..)
            .zip(self.iter_chronological().cloned())
            .map(|(i, y)| [i.into(), num_traits::cast(y).unwrap()].into())
            .collect()
    }
}
