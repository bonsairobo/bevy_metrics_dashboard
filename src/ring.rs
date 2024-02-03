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

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn latest(&self) -> Option<&T> {
        self.elements.back()
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
}
