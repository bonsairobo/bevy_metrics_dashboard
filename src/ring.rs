use bevy::prelude::default;
use either::Either;

/// A resizable ring buffer.
pub struct Ring<T> {
    elements: Vec<T>,
    // Always points to the most recently written element (except when nothing
    // has been written yet).
    cursor: usize,
}

impl<T: Copy + Default> Ring<T> {
    pub fn new(size: usize) -> Self {
        Self {
            elements: vec![default(); size],
            cursor: size - 1,
        }
    }

    pub fn resize(&mut self, new_size: usize) {
        let old_size = self.elements.len();

        if new_size == old_size {
            return;
        }
        if new_size == 0 {
            self.elements.truncate(0);
            return;
        }

        if new_size < old_size {
            self.shrink(new_size);
        } else {
            self.grow(new_size);
        }
    }

    fn grow(&mut self, new_size: usize) {
        let old_size = self.elements.len();
        let old_last_i = old_size - 1;
        let oldest_i = self.oldest_index();
        let oldest_value = self.elements[oldest_i];

        // Resize first then copy.
        self.elements.resize(new_size, default());

        // |----x----|---------|
        //      ^    ^         ^
        //      c    old       new

        if self.cursor == old_last_i {
            // No copying necessary.
            return;
        }

        // No risk of overwriting, just shift elements to create space for
        // the empty tail.
        let translate = new_size - old_size;
        self.elements
            .copy_within(oldest_i..old_size, oldest_i + translate);

        // Clear the new tail. Use the oldest value to prevent jarring changes
        // to the range.
        self.elements[oldest_i..old_size].fill(oldest_value);
    }

    fn shrink(&mut self, new_size: usize) {
        let old_size = self.elements.len();
        let new_last_i = new_size - 1;

        if self.cursor == new_last_i {
            // No copying necessary.
            self.elements.truncate(new_size);
            return;
        }

        // Need to copy elements first before they're truncated.
        if self.cursor < new_size {
            // |----x----|---------|
            //      ^    ^         ^
            //      c    new       old
            //
            // Overwrite oldest values first.
            let n_move = new_last_i - self.cursor;
            let translate = old_size - new_size;
            let dst_start = self.cursor + 1;
            self.translate_left(n_move, translate, dst_start);
        } else {
            // |---------|----x----|
            //           ^    ^    ^
            //           new  c    old
            let n_move = new_size;
            let translate = self.cursor - new_last_i;
            let dst_start = 0;
            self.translate_left(n_move, translate, dst_start);

            // Move the cursor in bounds.
            self.cursor = new_last_i;
        }

        self.elements.truncate(new_size);
    }

    /// Copy a range to the left by `translate`.
    fn translate_left(&mut self, n_move: usize, translate: usize, dst_start: usize) {
        let src_start = dst_start + translate;
        let src_end = src_start + n_move;
        self.elements.copy_within(src_start..src_end, dst_start);
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn latest(&self) -> &T {
        &self.elements[self.cursor]
    }

    pub fn oldest_index(&self) -> usize {
        (self.cursor + 1) % self.elements.len()
    }

    /// Overwrites the oldest element.
    pub fn push(&mut self, elem: T) {
        self.cursor = self.oldest_index();
        self.elements[self.cursor] = elem;
    }

    pub fn iter_chronological(&self) -> impl Iterator<Item = &T> {
        if self.oldest_index() == 0 {
            Either::Left(self.elements.iter())
        } else {
            Either::Right(
                self.elements[self.oldest_index()..]
                    .iter()
                    .chain(self.elements[..=self.cursor].iter()),
            )
        }
    }
}
