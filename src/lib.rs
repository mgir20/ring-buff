// A circular buffer is a linear data structure following the principle of FIFO (First In First Out).
// Instead of ending the queue at the last position, it starts from the last position after the last,
// making the queue behave like a circular data structure.
// Also called Ring buffer or circular queue
// All operations on the ring buffer are O(1)

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

        if self.writer == CAP - 1 {
            self.writer = 0
        } else {
            self.writer += 1;
        }
    }

    fn pop(&mut self) -> T {
        let reader = self.reader;

        if self.reader == CAP {
            self.reader = 0;
        } else {
            self.reader += 1;
        }

        self.data[reader].unwrap()
    }

    fn capacity(&self) -> usize {
        CAP
    }

    fn peek(&self) -> T {
        self.data[self.reader].unwrap()
    }

    fn empty() {}
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

    #[test]
    fn peek_returns_current_tail() {
        let mut buffer: RingBuff<i32, 10> = RingBuff::new();
        buffer.push_back(100);
        buffer.push_back(101);
        buffer.push_back(102);

        let element = buffer.peek();

        assert_eq!(element, 100);
    }

    #[test]
    fn peek_does_not_move_reader() {
        let mut buffer: RingBuff<i32, 10> = RingBuff::new();
        buffer.push_back(100);
        buffer.push_back(101);
        buffer.push_back(102);

        buffer.peek();
        let element = buffer.pop();

        assert_eq!(element, 100);
    }

    #[test]
    fn fill_and_overwrite_oldest_element() {
        let mut buffer: RingBuff<i32, 4> = RingBuff::new();
        // Fill
        buffer.push_back(100);
        buffer.push_back(101);
        buffer.push_back(102);
        buffer.push_back(103);

        // Overwrite oldest
        buffer.push_back(104);

        let element = buffer.pop();

        assert_eq!(element, 104);
    }

    #[test]
    fn pop_all_elements_in_order() {
        let mut buffer: RingBuff<i32, 4> = RingBuff::new();
        // Fill
        buffer.push_back(100);
        buffer.push_back(101);
        buffer.push_back(102);
        buffer.push_back(103);

        let (el1, el2, el3, el4) = (buffer.pop(), buffer.pop(), buffer.pop(), buffer.pop());

        println!("{},{},{},{}", el1, el2, el3, el4);


        assert_eq!((el1, el2, el3, el4), (100, 101, 102, 103));
    }
}
