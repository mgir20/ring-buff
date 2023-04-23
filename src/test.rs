#[cfg(test)]
mod tests {
    use crate::RingBuff;
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

        assert_eq!(element.unwrap(), 101);
    }

    #[test]
    fn overflow_and_pop_oldest_element() {
        let mut buffer: RingBuff<i32, 4> = RingBuff::new();
        // Fill
        buffer.push_back(100);
        buffer.push_back(101);
        buffer.push_back(102);
        buffer.push_back(103);

        // Overwrite oldest (100)
        buffer.push_back(104);

        let element = buffer.pop();
        println!("{:?}", buffer);

        // The updated oldest element is now 101
        assert_eq!(element.unwrap(), 101);
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

        assert_eq!([101, 102, 103, 104], result);
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

        assert_eq!([103, 104], result);
    }

    #[test]
    fn iterator_does_not_consume_elements() {
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

        for (i, val) in buffer.iter().enumerate() {
            assert_eq!(*val, 100 + i as i32)
        }
    }

    #[test]
    fn get_element() {
        let mut buffer: RingBuff<i32, 4> = RingBuff::new();

        buffer.push_back(100);
        buffer.push_back(101);
        buffer.push_back(102);

        let elements = (
            *buffer.get(0).unwrap(),
            *buffer.get(1).unwrap(),
            *buffer.get(2).unwrap()
        );

        assert_eq!(elements, (100, 101, 102));
    }

    #[test]
    fn get_element_outside_inner_range() {
        let mut buffer: RingBuff<i32, 4> = RingBuff::new();

        buffer.push_back(100);
        buffer.push_back(101);


        buffer.push_back(102);
        buffer.push_back(103);

        buffer.pop();
        buffer.pop();


        buffer.push_back(104);
        buffer.push_back(105);

        let element = *buffer.get(2).unwrap();

        assert_eq!(element, 104);
    }

    #[test]
    fn get_does_not_move_reader() {
        let mut buffer: RingBuff<i32, 10> = RingBuff::new();
        buffer.push_back(100);
        buffer.push_back(101);
        buffer.push_back(102);

        buffer.get(0);
        let element = buffer.pop();

        assert_eq!(element.unwrap(), 100);
    }

    #[test]
    fn clear_full_buffer() {
        let mut buffer: RingBuff<i32, 10> = RingBuff::new();
        buffer.push_back(100);
        buffer.push_back(101);
        buffer.push_back(102);
        buffer.push_back(103);

        buffer.clear();
        let mut empty = true;

        for val in buffer.data.iter() {
            empty = val.is_none();
        }

        assert_eq!(empty, true);
    }

    #[test]
    fn retain_mut_unaligned() {
        let mut buffer: RingBuff<i32, 10> = RingBuff::new();
        buffer.push_back(10);
        buffer.push_back(101);
        buffer.push_back(12);
        buffer.push_back(13);
        buffer.push_back(51);

        buffer.pop();
        buffer.pop();
        buffer.pop();

        buffer.push_back(351);
        buffer.push_back(250);
        buffer.push_back(25);
        buffer.push_back(25);
        buffer.push_back(25);
        buffer.push_back(25);
        buffer.push_back(25);

        buffer.retain_mut(|x| x < &mut 50);

        assert_eq!(buffer.reader, 3);
        assert_eq!(buffer.writer, 9);
        assert_eq!(buffer.len(), 6);
    }

    #[test]
    fn retain_mut_aligned() {
        let mut buffer: RingBuff<i32, 10> = RingBuff::new();
        buffer.push_back(10);
        buffer.push_back(101);
        buffer.push_back(12);
        buffer.push_back(13);
        buffer.push_back(51);


        buffer.push_back(351);
        buffer.push_back(250);
        buffer.push_back(25);
        buffer.push_back(25);
        buffer.push_back(25);
        buffer.push_back(25);
        buffer.push_back(25);


        buffer.retain_mut(|x| x < &mut 50);

        assert_eq!(buffer.reader, 2);
        assert_eq!(buffer.writer, 9);
        assert_eq!(buffer.len(), 7);
    }

    #[test]
    fn reassign_value_get_mut() {
        let mut buffer: RingBuff<i32, 10> = RingBuff::new();
        buffer.push_back(10);
        buffer.push_back(101);
        buffer.push_back(12);
        buffer.push_back(13);

        if let Some(val) = buffer.get_mut(2) {
            *val = 40
        }

        assert_eq!(buffer.data[2], Some(40));
    }

    #[test]
    fn index_conversion() {
        let mut buffer: RingBuff<i32, 5> = RingBuff::new();

        buffer.push_back(351);
        buffer.push_back(250);
        buffer.push_back(25);

        buffer.pop();
        buffer.pop();

        buffer.push_back(25);
        buffer.push_back(25);
        buffer.push_back(25);

        let i1 = buffer.relative_to_absolute_index(0);
        let i2 = buffer.relative_to_absolute_index(1);
        let i3 = buffer.relative_to_absolute_index(2);
        let i4 = buffer.relative_to_absolute_index(3);

        assert_eq!((i1, i2, i3, i4), (Some(2), Some(3), Some(4), Some(0)));
    }
}