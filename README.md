# Ring Buffer

A circular buffer is a queue structure following the principle of First In First Out (FIFO)
It is circular because elements are stored in a contiguous manner, but when full, the oldest element
in the queue is overwritten with the newest one, resulting in a structure that cannot overflow.

It is useful for storing unpredictable input where some data can be lost, for example keyboard input.
If set to a manageable capacity, it makes it possible to handle large data sets without using an
unreasonable amount of memory.

## Structure

[Drawing]

## Comments

This project was done quickly after reading the Rust Book as a way to put into practice the knowledge acquired.