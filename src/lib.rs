// A circular buffer is a linear data structure following the principle of FIFO (First In First Out).
// Instead of ending the queue at the last position, it starts from the last position after the last,
// making the queue behave like a circular data structure.
// Also called Ring buffer or circular queue

use std::fs::read;

struct RingBuff<T, const CAP: usize> {
    data: [Option<T>; CAP],
    // N is the capacity of the ring buffer
    reader: usize,
    writer: usize,
}

impl<T: Default + Clone + Copy, const CAP: usize> RingBuff<T, CAP> {
    fn new() -> Self {
        Self {
            data: [None; CAP],
            reader: 0,
            writer: 0,
        }
    }

    fn push_back(&mut self, element: T) {
        // When reaching the end of the allocated data sequence,
        // the data is written on the first cell
        self.data[self.writer] = Some(element);

        if self.writer == CAP {
            self.writer = 0
        } else {
            self.writer += 1;
        }
    }

    fn pop(&mut self) -> T {
        let reader = self.reader;

        if self.reader == 0 {
            self.reader = CAP - 1;
        } else {
            self.reader -= 1;
        }

        self.data[reader].unwrap()
    }

    fn capacity(&self) -> usize {
        CAP
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_construct_ring_buffer() {
        let buffer: RingBuff<i32, 10> = RingBuff::new();
    }


    #[test]
    fn push_pop_one_element() {
        let mut buffer: RingBuff<i32, 10> = RingBuff::new();
        buffer.push_back(100);
        let element = buffer.pop();

        assert_eq!(element, 100);
    }

    #[test]
    fn push_multiple_pop_one_element() {
        let mut buffer: RingBuff<i32, 10> = RingBuff::new();
        buffer.push_back(100);
        buffer.push_back(101);
        buffer.push_back(102);
        let element = buffer.pop();

        assert_eq!(element, 100);
    }

    #[test]
    fn check_capacity() {
        let mut buffer: RingBuff<i32, 10> = RingBuff::new();
        let capacity = buffer.capacity();

        assert_eq!(capacity, 10);
    }
}
