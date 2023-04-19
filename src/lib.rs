// A circular buffer is a linear data structure following the principle of FIFO (First In First Out).
// Instead of ending the queue at the last position, it starts from the last position after the last,
// making the queue behave like a circular data structure.
// Also called Ring buffer or circular queue
// All operations on the ring buffer are O(1)
pub struct RingBuff<T, const CAP: usize> {
    data: [Option<T>; CAP],
    reader: usize,
    writer: usize,
    size: usize,
}

impl<T, const CAP: usize> RingBuff<T, CAP> {
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

    pub fn push_back(&mut self, element: T) {
        // When reaching the end of the allocated data sequence,
        // the data is written on the first cell
        self.data[self.writer] = Some(element);
        if !self.is_full() {
            self.size += 1;
        }

        self.writer = self.next_index(self.writer);
    }

    pub fn pop(&mut self) -> Option<T> {
        let reader = self.reader;
        self.reader = self.next_index(self.reader);

        self.size -= 1;
        std::mem::take(&mut self.data[reader])
    }

    pub(crate) fn next_index(&self, index: usize) -> usize {
        if index == CAP - 1 {
            0
        } else {
            index + 1
        }
    }

    pub const fn capacity(&self) -> usize {
        CAP
    }

    const fn size(&self) -> usize {
        self.size
    }

    const fn is_empty(&self) -> bool {
        self.size == 0
    }

    const fn is_full(&self) -> bool {
        self.size == CAP
    }

    // Return the next value to be read without consuming it
    pub fn peek(&self) -> Option<&T> {
        self.data[self.reader].as_ref()
    }

    pub fn iter(&self) -> RingBuffIter<T, CAP> {
        RingBuffIter {
            buffer: &self,
            index: self.reader,
            count: 0,
        }
    }
}

pub struct RingBuffIter<'a, T, const CAP: usize> {
    buffer: &'a RingBuff<T, CAP>,
    index: usize,
    count: usize,
}

impl<'a, T, const CAP: usize> Iterator for RingBuffIter<'a, T, CAP> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.buffer.size {
            None
        } else {
            let current = &self.buffer.data[self.index];
            self.index = self.buffer.next_index(self.index);
            self.count += 1;
            current.as_ref()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_construct_ring_buffer() {
        let _buffer: RingBuff<i32, 10> = RingBuff::new();
    }


    #[test]
    fn push_pop_one_element() {
        let mut buffer: RingBuff<i32, 10> = RingBuff::new();
        buffer.push_back(100);
        let element = buffer.pop();

        assert_eq!(element.unwrap(), 100);
    }

    #[test]
    fn push_multiple_pop_one_element() {
        let mut buffer: RingBuff<i32, 10> = RingBuff::new();
        buffer.push_back(100);
        buffer.push_back(101);
        buffer.push_back(102);
        let element = buffer.pop();

        assert_eq!(element.unwrap(), 100);
    }

    #[test]
    fn check_capacity() {
        let buffer: RingBuff<i32, 10> = RingBuff::new();
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

        assert_eq!(*element.unwrap(), 100);
    }

    #[test]
    fn peek_does_not_move_reader() {
        let mut buffer: RingBuff<i32, 10> = RingBuff::new();
        buffer.push_back(100);
        buffer.push_back(101);
        buffer.push_back(102);

        buffer.peek();
        let element = buffer.pop();

        assert_eq!(element.unwrap(), 100);
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

        assert_eq!(element.unwrap(), 104);
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

        println!("{},{},{},{}", el1.unwrap(), el2.unwrap(), el3.unwrap(), el4.unwrap());


        assert_eq!((el1.unwrap(), el2.unwrap(), el3.unwrap(), el4.unwrap()), (100, 101, 102, 103));
    }

    #[test]
    fn iterate_through() {
        let mut buffer: RingBuff<i32, 4> = RingBuff::new();

        // Fill
        buffer.push_back(100);
        buffer.push_back(101);
        buffer.push_back(102);
        buffer.push_back(103);

        let mut result = [0, 0, 0, 0];

        for (i, val) in buffer.iter().enumerate() {
            result[i] = *val;
        }

        assert_eq!([100, 101, 102, 103], result);
    }

    #[test]
    fn overflow_then_iterate_through() {
        let mut buffer: RingBuff<i32, 4> = RingBuff::new();

        // Fill
        buffer.push_back(100);
        buffer.push_back(101);
        buffer.push_back(102);
        buffer.push_back(103);
        buffer.push_back(104);


        let mut result = [0, 0, 0, 0];

        for (i, val) in buffer.iter().enumerate() {
            result[i] = *val;
        }

        assert_eq!([104, 101, 102, 103], result);
    }

    #[test]
    fn push_pop_iterate_through() {
        let mut buffer: RingBuff<i32, 4> = RingBuff::new();

        // Fill
        buffer.push_back(100);

        buffer.push_back(101);
        buffer.push_back(102);
        buffer.push_back(103);
        buffer.push_back(104);

        buffer.pop();
        buffer.pop();


        let mut result = [0, 0];

        for (i, val) in buffer.iter().enumerate() {
            result[i] = *val;
        }

        assert_eq!([102, 103], result);
    }
}
