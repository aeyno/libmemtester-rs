# libmemtester-rs

![pipeline](https://github.com/aeyno/libmemtester-rs/actions/workflows/rust.yml/badge.svg)

This project is a Rust library for testing memory. It is a Rust version of [memtester](https://pyropus.ca./software/memtester/).

The goal of this crate is to provide a cross-platform memory tester (it has been tested on Linux and Windows) that can be used in other projects.

It works by allocating a large chunk of memory and then writing a pattern to it. It then reads the memory and checks that the pattern is correct. It does this for a number of different patterns. If the memory is faulty, it will detect it and return a hashmap of the faulty addresses and the number of times they were faulty.

To prevent the operating system from swapping out the memory and moving pages, this crate locks the memory (using `mlock` syscall on Linux and `VirtualLock` on Windows).
In order to do this, the crate needs to be run as root.

## ⌨ Usage

```rust
use libmemtester::MemoryTests;

fn main() {
    // The size to allocate in bytes (must be divisible by 16)
    let size_to_allocate = 1024 * 1024 * 1024; // 1GB

    let debug_prints = false; // If you want the debug prints

    // Create a new MemTester instance
    let mem_tests_res = MemoryTests::new(size_to_allocate, debug_prints);

    // Checking if there were any errors when allocating the memory
    if let Err(e) = mem_tests {
        println!("Error: {}", e);
        return;
    }
    let tests = mem_tests.unwrap();

    // We run the tests by calling an iterator
    // This way you can stop the tests when one fails
    // or simply display information as the tests goes along
    for let Some((test_name, error_count)) in tests.get_iterator() {
        println!("Test: {} - Error count: {}", test_name, error_count);
    }

    // When the iterator is done, we can get the results
    // This will return a hashmap of the faulty addresses and the number of times they were faulty
    let errors = tests.get_errors();
    println!("Errors: {:#?}", errors);
}
```

You can see an example of this crate being used in [memtester-rs](https://github.com/aeyno/memtester-rs).

## 📃 License

This project is licensed under the GPLv3 licence - see the [LICENSE](LICENSE) file for details
