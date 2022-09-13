# libmemtester-rs

![pipeline](https://github.com/aeyno/libmemtester-rs/actions/workflows/rust.yml/badge.svg)

This project is a Rust library for testing memory. It is a Rust version of [memtester](https://pyropus.ca./software/memtester/).

The goal of this crate is to provide a cross-platform memory tester (it has been tested on Linux and Windows) that can be used in other projects.

It works by allocating a large chunk of memory and then writing a pattern to it. It then reads the memory and checks that the pattern is correct. It does this for a number of different patterns. If the memory is faulty, it will detect it and return a hashmap of the faulty addresses and the number of times they were faulty.
