//! Simple circular Buffer implementation
//!
//! A circular buffer is a linear data structure following the principle of FIFO (First In First Out).
//! Instead of ending the queue at the last position, it starts from the last position after the last,
//! making the queue behave like a circular data structure.
//!
//!
//! To be used when losing data is acceptable,
//! All basic operations on the ring buffer are O(1)
//! Also called Ring buffer or circular queue
//! The implementation stores data on the stack, for blablalbal
//! It should not be used to store too large data sets, since it could cause an overflow

use std::fmt::{Debug};
use std::mem;

mod test;

/// Ring buffer implementation
#[derive(Debug)]
pub struct RingBuff<T, const CAP: usize> {
    /// The data is stored in an array
    data: [Option<T>; CAP],
    /// The queue head
    reader: usize,
    /// The queue tail
    writer: usize,
    /// Number of elements in the queue
    size: usize,
}

impl<T, const CAP: usize> RingBuff<T, CAP> {
    /// Return a new Ring Buffer
    ///
    /// # Arguments
    ///
    /// # Examples
    /// `let buffer: RingBuff<i32, 4> = RingBuff::new();`
    ///
    pub fn new() -> Self
        where
            T: Copy, {
        Self {
            data: [None; CAP],
            reader: 0,
            writer: 0,
            size: 0,
        }
    }

    /// Pushes one element to the back of the queue.
    ///
    /// # Arguments
    /// * `element` - The element to add to the queue
    pub fn push_back(&mut self, element: T) {
        // When reaching the end of the allocated data sequence,
        // the data is written on the first cell

        if self.is_full() { self.reader = self.next_index(self.reader); }

        self.data[self.writer] = Some(element);

        self.size += !self.is_full() as usize;
        self.writer = self.next_index(self.writer);
    }

    /// Remove one element from the back of the queue
    /// and returns it.
    ///
    /// # Arguments
    ///
    pub fn pop(&mut self) -> Option<T> {
        let reader = self.reader;
        self.reader = self.next_index(self.reader);

        self.size -= 1;
        mem::take(&mut self.data[reader])
    }

    /// Returns the index of the next element in data.
    ///
    /// # Arguments
    ///
    /// * `index` - The original index
    ///
    pub(crate) fn next_index(&self, index: usize) -> usize {
        if index == CAP - 1 {
            0
        } else {
            index + 1
        }
    }

    fn previous_index(&self, index: usize) -> usize {
        if index == 0 {
            CAP - 1
        } else {
            index - 1
        }
    }

    fn relative_to_absolute_index(&self, index: usize) -> Option<usize> {
        if index >= self.len() {
            None
        } else if self.reader + index >= self.capacity() {
            Some((self.reader + index) % self.capacity())
        } else {
            Some(self.reader + index)
        }
    }

    /// Retains only elements fitting a predicate.
    ///
    /// # Arguments
    ///
    ///  * `f` - A predicate
    ///
    pub fn retain<F>(&mut self, mut f: F)
        where
            F: FnMut(&T) -> bool,
    {
        self.retain_mut(|elem| f(elem));
    }

    /// Retains only elements fitting a predicate,
    /// passing a mutable reference to it.
    ///
    /// # Arguments
    ///
    ///  * `f` - A predicate
    ///
    pub fn retain_mut<F>(&mut self, mut f: F)
        where
            F: FnMut(&mut T) -> bool,
    {
        let mut size = self.len();


        for i in 0..self.len() {
            if !f(self.get_mut(i).unwrap()) {
                self.data[self.relative_to_absolute_index(i).unwrap()] = None;
                self.writer = self.previous_index(self.writer);
                size -= 1;
            }
        }

        for i in 0..self.len() {
            if self.get_mut(i).is_none() {
                for j in i..self.len() {
                    if self.get_mut(j).is_some() {
                        let idx = self.relative_to_absolute_index(i).unwrap();
                        let jdx = self.relative_to_absolute_index(j).unwrap();
                        self.data.swap(idx, jdx);
                        break;
                    }
                }
            }
        }

        self.size = size;
    }

    /// Removes all elements in the buffer.
    /// Note that this method has no effect on
    /// the allocated capacity of the buffer.
    ///
    /// # Arguments
    ///
    pub fn clear(&mut self) {
        for _ in 0..self.size {
            self.pop();
        }
    }

    /// Returns true if the buffer contains no elements.
    ///
    /// # Arguments
    ///
    pub const fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns whether or not the buffer is full.
    ///
    /// # Arguments
    ///
    const fn is_full(&self) -> bool {
        self.size == CAP
    }

    /// Returns the number of elements in the buffer.
    ///
    /// # Arguments
    ///
    const fn len(&self) -> usize {
        self.size
    }

    /// Returns the maximum number of elements the
    /// buffer can hold.
    ///
    /// # Arguments
    ///
    pub const fn capacity(&self) -> usize {
        CAP
    }

    /// Returns a reference to an element or None
    /// if the index is out of bounds.
    ///
    /// # Arguments
    /// * `index` - Position of the element to look up
    ///
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len() {
            None
        } else {
            let i = self.relative_to_absolute_index(index).expect("Index is valid.");
            self.data[i].as_ref()
        }
    }

    /// Returns a mutable reference to an element or None
    /// if the index is out of bounds
    ///
    /// # Arguments
    /// * `index` - Position of the element to look up
    ///
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len() {
            None
        } else {
            let i = self.relative_to_absolute_index(index).expect("Index is valid.");
            self.data[i].as_mut()
        }
    }

    /// Returns an iterator on the buffer
    ///
    /// # Arguments
    ///
    pub fn iter(&self) -> RingBuffIter<T, CAP> {
        RingBuffIter {
            buffer: &self,
            index: self.reader,
            count: 0,
        }
    }

    /*    /// Returns a mutable iterator on the buffer
        ///
        /// # Arguments
        ///
        pub fn iter_mut(&mut self) -> RingBuffIterMut<T, CAP> {
            RingBuffIterMut {
                buffer: &mut self,
                index: self.reader,
                count: 0,
            }
        }*/
}

pub struct RingBuffIter<'a, T, const CAP: usize> {
    /// A reference to the RingBuff
    buffer: &'a RingBuff<T, CAP>,
    /// The index of the iterator in the buffer data array
    index: usize,
    /// Count of elements iterated through
    count: usize,
}

impl<'a, T, const CAP: usize> Iterator for RingBuffIter<'a, T, CAP> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.buffer.len() {
            None
        } else {
            let current = &self.buffer.data[self.index];
            self.index = self.buffer.next_index(self.index);
            self.count += 1;
            current.as_ref()
        }
    }
}

/*pub struct RingBuffIterMut<'a, T, const CAP: usize> {
    /// A reference to the RingBuff
    buffer: &'a mut RingBuff<T, CAP>,
    /// The index of the iterator in the buffer data array
    index: usize,
    /// Count of elements iterated through
    count: usize,
}

impl<'a, T, const CAP: usize> Iterator for RingBuffIterMut<'a, T, CAP> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.buffer.len() {
            None
        } else {
            let current = &mut self.buffer.data[self.index];
            self.index = self.buffer.next_index(self.index);
            self.count += 1;
            current.as_mut()
        }
    }
}*/
