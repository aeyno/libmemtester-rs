use core::ffi::c_void;
#[cfg(target_os = "linux")]
use libc::{mlock, munlock};
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
        println!("Locking memory using VirtualLock...");
        let ptr = hog.as_ptr() as *mut c_void;
        unsafe {
            SetProcessWorkingSetSize(GetCurrentProcess(), size + 400 * 1024, size + 800 * 1024);
        };
        let ret = unsafe { VirtualLock(ptr, size) };
        assert_ne!(ret, 0);
        println!("Memory locked!");
    }
    #[cfg(target_os = "linux")]
    {
        println!("Locking memory using mlock...");
        let ptr = pointer as *const c_void;
        let ret = unsafe { mlock(ptr, size) };
        assert_eq!(ret, 0);
        println!("Memory locked!");
    }
}

/// Unlock the current process's memory.
fn unlock_mem(pointer: *mut c_void, size: usize) {
    #[cfg(windows)]
    {
        println!("Unlocking memory using VirtualUnlock...");
        let ptr = pointer as *mut c_void;
        let ret = unsafe { VirtualUnlock(ptr, size) };
        assert_ne!(ret, 0);
        println!("Memory unlocked!");
    }
    #[cfg(target_os = "linux")]
    {
        println!("Unlocking memory using munlock...");
        let ptr = pointer as *const c_void;
        let ret = unsafe { munlock(ptr, size) };
        assert_eq!(ret, 0);
        println!("Memory unlocked!");
    }
}

fn compare_halfs(hog: &[u64], errors: &mut HashMap<usize, u32>) -> Result<(), usize> {
    let mut error_count = 0;
    let half = hog.len() / 2;
    for i in 0..half {
        if hog[i] != hog[half + i] {
            let address = hog.as_ptr() as usize + 8 * i;
            error_count += 1;
            errors.entry(address).and_modify(|e| *e += 1).or_insert(1);
            println!(
                "Mismatch at index {} (Logical Adress: {:#0x}): {:#0x} != {:#0x}",
                i,
                address,
                hog[i],
                hog[half + i]
            );
        }
    }
    if error_count == 0 {
        Ok(())
    } else {
        Err(error_count)
    }
}

fn test_rand_data(hog: &mut [u64], errors: &mut HashMap<usize, u32>) -> Result<(), usize> {
    let half = hog.len() / 2;
    let mut data = rand::random();
    for i in 0..half {
        hog[i] = data;
        hog[half + i] = data;
        data = rand::random();
    }

    compare_halfs(hog, errors)
}

fn test_xor(hog: &mut [u64], errors: &mut HashMap<usize, u32>) -> Result<(), usize> {
    let half = hog.len() / 2;
    let data: u64 = rand::random();
    for i in 0..half {
        hog[i] ^= data;
        hog[half + i] ^= data;
    }

    compare_halfs(hog, errors)
}

fn test_add(hog: &mut [u64], errors: &mut HashMap<usize, u32>) -> Result<(), usize> {
    let half = hog.len() / 2;
    let data: u64 = rand::random();
    for i in 0..half {
        hog[i] = hog[i].saturating_add(data);
        hog[half + i] = hog[half + i].saturating_add(data);
    }

    compare_halfs(hog, errors)
}

fn test_sub(hog: &mut [u64], errors: &mut HashMap<usize, u32>) -> Result<(), usize> {
    let half = hog.len() / 2;
    let data: u64 = rand::random();
    for i in 0..half {
        hog[i] = hog[i].saturating_sub(data);
        hog[half + i] = hog[half + i].saturating_sub(data);
    }

    compare_halfs(hog, errors)
}

fn test_mul(hog: &mut [u64], errors: &mut HashMap<usize, u32>) -> Result<(), usize> {
    let half = hog.len() / 2;
    let data: u64 = rand::random();
    for i in 0..half {
        hog[i] = hog[i].saturating_mul(data);
        hog[half + i] = hog[half + i].saturating_mul(data);
    }

    compare_halfs(hog, errors)
}

fn test_div(hog: &mut [u64], errors: &mut HashMap<usize, u32>) -> Result<(), usize> {
    let half = hog.len() / 2;
    let data: u64 = rand::random();
    for i in 0..half {
        hog[i] = hog[i].saturating_div(data);
        hog[half + i] = hog[half + i].saturating_div(data);
    }

    compare_halfs(hog, errors)
}

fn test_or(hog: &mut [u64], errors: &mut HashMap<usize, u32>) -> Result<(), usize> {
    let half = hog.len() / 2;
    let data: u64 = rand::random();
    for i in 0..half {
        hog[i] |= data;
        hog[half + i] |= data;
    }

    compare_halfs(hog, errors)
}

fn test_and(hog: &mut [u64], errors: &mut HashMap<usize, u32>) -> Result<(), usize> {
    let half = hog.len() / 2;
    let data: u64 = rand::random();
    for i in 0..half {
        hog[i] &= data;
        hog[half + i] &= data;
    }

    compare_halfs(hog, errors)
}

fn test_solidbits(hog: &mut [u64], errors: &mut HashMap<usize, u32>) -> Result<(), usize> {
    let half = hog.len() / 2;
    let mut error_count = 0;
    let mut data: u64;
    for i in 0..64 {
        print!("\n\t{} of 64... ", i);
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
        if let Err(count) = compare_halfs(hog, errors) {
            error_count += count;
        }
    }
    match error_count {
        0 => Ok(()),
        _ => Err(error_count),
    }
}

fn test_checkboard(hog: &mut [u64], errors: &mut HashMap<usize, u32>) -> Result<(), usize> {
    let half = hog.len() / 2;
    let mut error_count = 0;
    let mut data: u64;
    for i in 0..64 {
        print!("\n\t{} of 64... ", i);
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
        if let Err(count) = compare_halfs(hog, errors) {
            error_count += count;
        }
    }
    match error_count {
        0 => Ok(()),
        _ => Err(error_count),
    }
}

fn main() {
    let allocation_amount: i64 = 4096;

    let amount_as_u8: usize = (allocation_amount * 1024 * 1024).try_into().unwrap();

    // Since we are using 64 bit values, we need to divide the amount by 8
    let hog_length = amount_as_u8 / 8;
    let mut hog = vec![0u64; hog_length];

    let ptr = hog.as_ptr() as *mut c_void;
    lock_mem(ptr, amount_as_u8);

    let tests: Vec<(
        &str,
        fn(&mut [u64], &mut HashMap<usize, u32>) -> Result<(), usize>,
    )> = vec![
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

    // We store the errors in a hashmap and we count the number of error for each memory location
    let mut errors: HashMap<usize, u32> = HashMap::new();

    for (name, test) in tests {
        print!("Test {}... ", name);
        match test(&mut hog, &mut errors) {
            Ok(_) => println!("OK"),
            Err(count) => println!("{} error(s)", count),
        }
    }

    let mut error_count = 0;

    println!("All tests completed.");

    for (index, count) in &errors {
        println!("Error at index {:#0x}: {}", index, count);
        error_count += count;
    }
    println!("{} errors total", error_count);

    unlock_mem(ptr, amount_as_u8);
    drop(hog);
}
