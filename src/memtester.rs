use core::ffi::c_void;
#[cfg(target_os = "linux")]
use libc::{mlock, munlock};
use nix::unistd::Uid;
use std::collections::HashMap;
#[cfg(windows)]
use winapi::{
    ctypes::c_void,
    um::memoryapi::{VirtualLock, VirtualUnlock},
    um::processthreadsapi::GetCurrentProcess,
    um::winbase::SetProcessWorkingSetSize,
};

/// Lock the current process's memory to prevent swapping.
fn lock_mem(pointer: *mut c_void, size: usize) {
    #[cfg(windows)]
    {
        // Locking memory using VirtualLock
        let ptr = hog.as_ptr() as *mut c_void;
        unsafe {
            SetProcessWorkingSetSize(GetCurrentProcess(), size + 400 * 1024, size + 800 * 1024);
        };
        let ret = unsafe { VirtualLock(ptr, size) };
        assert_ne!(ret, 0);
    }
    #[cfg(target_os = "linux")]
    {
        //Locking memory using mlock
        let ptr = pointer as *const c_void;
        let ret = unsafe { mlock(ptr, size) };
        assert_eq!(ret, 0);
    }
}

/// Unlock the current process's memory.
fn unlock_mem(pointer: *mut c_void, size: usize) {
    #[cfg(windows)]
    {
        //Unlocking memory using VirtualUnlock...
        let ptr = pointer as *mut c_void;
        let ret = unsafe { VirtualUnlock(ptr, size) };
        assert_ne!(ret, 0);
    }
    #[cfg(target_os = "linux")]
    {
        //Unlocking memory using munlock...
        let ptr = pointer as *const c_void;
        let ret = unsafe { munlock(ptr, size) };
        assert_eq!(ret, 0);
    }
}

type TestFunction = fn(&mut [u64], &mut HashMap<usize, u32>, bool) -> Result<(), usize>;

fn compare_halfs(
    hog: &[u64],
    errors: &mut HashMap<usize, u32>,
    debug_prints: bool,
) -> Result<(), usize> {
    let mut error_count = 0;
    let half = hog.len() / 2;
    for i in 0..half {
        if hog[i] != hog[half + i] {
            let address = hog.as_ptr() as usize + 8 * i;
            error_count += 1;
            errors.entry(address).and_modify(|e| *e += 1).or_insert(1);
            if debug_prints {
                println!(
                    "Mismatch at index {} (Logical Adress: {:#0x}): {:#0x} != {:#0x}",
                    i,
                    address,
                    hog[i],
                    hog[half + i]
                );
            }
        }
    }
    if error_count == 0 {
        Ok(())
    } else {
        Err(error_count)
    }
}

fn test_rand_data(
    hog: &mut [u64],
    errors: &mut HashMap<usize, u32>,
    debug_prints: bool,
) -> Result<(), usize> {
    let half = hog.len() / 2;
    let mut data = rand::random();
    for i in 0..half {
        hog[i] = data;
        hog[half + i] = data;
        data = rand::random();
    }

    compare_halfs(hog, errors, debug_prints)
}

fn test_xor(
    hog: &mut [u64],
    errors: &mut HashMap<usize, u32>,
    debug_prints: bool,
) -> Result<(), usize> {
    let half = hog.len() / 2;
    let data: u64 = rand::random();
    for i in 0..half {
        hog[i] ^= data;
        hog[half + i] ^= data;
    }

    compare_halfs(hog, errors, debug_prints)
}

fn test_add(
    hog: &mut [u64],
    errors: &mut HashMap<usize, u32>,
    debug_prints: bool,
) -> Result<(), usize> {
    let half = hog.len() / 2;
    let data: u64 = rand::random();
    for i in 0..half {
        hog[i] = hog[i].saturating_add(data);
        hog[half + i] = hog[half + i].saturating_add(data);
    }

    compare_halfs(hog, errors, debug_prints)
}

fn test_sub(
    hog: &mut [u64],
    errors: &mut HashMap<usize, u32>,
    debug_prints: bool,
) -> Result<(), usize> {
    let half = hog.len() / 2;
    let data: u64 = rand::random();
    for i in 0..half {
        hog[i] = hog[i].saturating_sub(data);
        hog[half + i] = hog[half + i].saturating_sub(data);
    }

    compare_halfs(hog, errors, debug_prints)
}

fn test_mul(
    hog: &mut [u64],
    errors: &mut HashMap<usize, u32>,
    debug_prints: bool,
) -> Result<(), usize> {
    let half = hog.len() / 2;
    let data: u64 = rand::random();
    for i in 0..half {
        hog[i] = hog[i].saturating_mul(data);
        hog[half + i] = hog[half + i].saturating_mul(data);
    }

    compare_halfs(hog, errors, debug_prints)
}

fn test_div(
    hog: &mut [u64],
    errors: &mut HashMap<usize, u32>,
    debug_prints: bool,
) -> Result<(), usize> {
    let half = hog.len() / 2;
    let data: u64 = rand::random();
    for i in 0..half {
        hog[i] = hog[i].saturating_div(data);
        hog[half + i] = hog[half + i].saturating_div(data);
    }

    compare_halfs(hog, errors, debug_prints)
}

fn test_or(
    hog: &mut [u64],
    errors: &mut HashMap<usize, u32>,
    debug_prints: bool,
) -> Result<(), usize> {
    let half = hog.len() / 2;
    let data: u64 = rand::random();
    for i in 0..half {
        hog[i] |= data;
        hog[half + i] |= data;
    }

    compare_halfs(hog, errors, debug_prints)
}

fn test_and(
    hog: &mut [u64],
    errors: &mut HashMap<usize, u32>,
    debug_prints: bool,
) -> Result<(), usize> {
    let half = hog.len() / 2;
    let data: u64 = rand::random();
    for i in 0..half {
        hog[i] &= data;
        hog[half + i] &= data;
    }

    compare_halfs(hog, errors, debug_prints)
}

fn test_solidbits(
    hog: &mut [u64],
    errors: &mut HashMap<usize, u32>,
    debug_prints: bool,
) -> Result<(), usize> {
    let half = hog.len() / 2;
    let mut error_count = 0;
    let mut data: u64;
    for i in 0..64 {
        if debug_prints {
            println!("\t{} of 64... ", i + 1);
        }
        data = if i % 2 == 0 {
            0xffffffffffffffff
        } else {
            0x0000000000000000
        };
        for j in 0..half {
            if i % 2 == 0 {
                hog[j] = data;
                hog[half + j] = data;
            } else {
                hog[j] = !data;
                hog[half + j] = !data;
            }
        }
        if let Err(count) = compare_halfs(hog, errors, debug_prints) {
            error_count += count;
        }
    }
    match error_count {
        0 => Ok(()),
        _ => Err(error_count),
    }
}

fn test_checkboard(
    hog: &mut [u64],
    errors: &mut HashMap<usize, u32>,
    debug_prints: bool,
) -> Result<(), usize> {
    let half = hog.len() / 2;
    let mut error_count = 0;
    let mut data: u64;
    for i in 0..64 {
        if debug_prints {
            println!("\t{} of 64... ", i + 1);
        }
        data = if i % 2 == 0 {
            0x5555555555555555
        } else {
            0xaaaaaaaaaaaaaaaa
        };
        for j in 0..half {
            if i % 2 == 0 {
                hog[j] = data;
                hog[half + j] = data;
            } else {
                hog[j] = !data;
                hog[half + j] = !data;
            }
        }
        if let Err(count) = compare_halfs(hog, errors, debug_prints) {
            error_count += count;
        }
    }
    match error_count {
        0 => Ok(()),
        _ => Err(error_count),
    }
}

const TESTS: [(&str, TestFunction); 10] = [
    ("Random Data", test_rand_data),
    ("XOR", test_xor),
    ("ADD", test_add),
    ("SUB", test_sub),
    ("MUL", test_mul),
    ("DIV", test_div),
    ("OR", test_or),
    ("AND", test_and),
    ("Solid Bits", test_solidbits),
    ("Checkboard", test_checkboard),
];

pub struct MemoryTestIterator<'a> {
    info_prints: bool,
    hog: &'a mut Vec<u64>,
    errors: &'a mut HashMap<usize, u32>,
    idx: usize,
}

impl Iterator for MemoryTestIterator<'_> {
    type Item = (String, usize);

    /// Run the next test and returns its name and result.
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= TESTS.len() {
            return None;
        }
        let (name, test) = TESTS[self.idx];
        let res = test(self.hog, self.errors, self.info_prints);
        self.idx += 1;
        match res {
            Ok(_) => Some((String::from(name), 0)),
            Err(count) => Some((String::from(name), count)),
        }
    }
}

impl MemoryTestIterator<'_> {
    pub fn next_test_name(&self) -> Option<&str> {
        if self.idx >= TESTS.len() {
            return None;
        }
        let (name, _) = TESTS[self.idx];
        Some(name)
    }
}

/// An object to run memory tests on a memory region.
/// The memory region is allocated as a vector of `u64` and is protected from swapping via a syscall
/// (`mlock` on Linux and `VirtualLock` on Windows).
pub struct MemoryTests {
    /// Activate debug prints
    info_prints: bool,
    /// The allocated chunk of memory
    hog: Vec<u64>,
    /// The errors found while testing
    errors: HashMap<usize, u32>,
}

impl MemoryTests {
    /// Create a new `MemoryTests` object.
    ///
    /// # Arguments
    /// - `allocation_amount`: the size, in bytes, of the memory region to allocate, should be a multiple of 16
    /// - `info_prints`: activate debug prints
    pub fn new(allocation_amount: usize, print_information: bool) -> Result<MemoryTests, String> {
        if !Uid::effective().is_root() {
            return Err("You must be root to run this test.".into());
        }
        if allocation_amount % 16 != 0 {
            return Err("Allocation amount must be a multiple of 16.".into());
        }
        let amount_as_u8: usize = allocation_amount;

        // Since we are using 64 bit values, we need to divide the amount by 8
        let hog_length = amount_as_u8 / 8;
        let hog = vec![0u64; hog_length];

        let ptr = hog.as_ptr() as *mut c_void;
        if print_information {
            println!("Locking memory...");
        }
        lock_mem(ptr, amount_as_u8);
        if print_information {
            println!("Memory locked.");
        }

        Ok(MemoryTests {
            info_prints: print_information,
            hog,
            errors: HashMap::new(),
        })
    }

    /// Returns an iterator of tests
    pub fn get_iterator(&mut self) -> MemoryTestIterator {
        MemoryTestIterator {
            info_prints: self.info_prints,
            hog: &mut self.hog,
            errors: &mut self.errors,
            idx: 0,
        }
    }

    /// Returns the errors found while testing
    pub fn get_errors(&self) -> &HashMap<usize, u32> {
        &self.errors
    }

    /// Returns the number of errors found while testing
    pub fn get_error_count(&self) -> u32 {
        let mut total = 0;
        for (_, count) in self.errors.iter() {
            total += *count;
        }
        total
    }

    /// Returns the allocated memory size in bytes
    pub fn get_allocated_size(&self) -> usize {
        self.hog.len() * 8
    }
}

impl Drop for MemoryTests {
    fn drop(&mut self) {
        if self.info_prints {
            println!("Unlocking memory...");
        }
        unlock_mem(self.hog.as_ptr() as *mut c_void, self.get_allocated_size());
        if self.info_prints {
            println!("Memory unlocked.");
        }
    }
}
