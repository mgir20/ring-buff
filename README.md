# Ring Buffer

A circular buffer is a queue structure following the principle of First In First Out (FIFO).
The name comes from the way the elements are stored in a contiguous manner in the structure and
deleted from the oldest one when full. This results in a structure that can never overflow

It is useful for storing unpredictable input where some data can be lost, for example keyboard input.
If set to a manageable capacity, it makes it possible to handle large data sets without using an
unreasonable amount of memory.

## Structure

[Drawing]

## Comments

This project was done to put into practice what I've learnt after reading the Rust Book.