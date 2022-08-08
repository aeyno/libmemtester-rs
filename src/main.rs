use core::ffi::c_void;
#[cfg(target_os = "linux")]
use libc::{mlock, munlock};
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

fn main() {
    let allocation_amount: i64 = 200;

    println!("Press enter to allocate.");
    let mut key = String::new();
    let _input = std::io::stdin().read_line(&mut key).unwrap();

    let amount_as_u8: usize = (allocation_amount * 1024 * 1024).try_into().unwrap();

    let mut hog = vec![0u8; amount_as_u8];

    let ptr = hog.as_ptr() as *mut c_void;
    lock_mem(ptr, amount_as_u8);

    hog[0..amount_as_u8].fill(1);

    assert_eq!(hog[amount_as_u8 - 8..], [1u8; 8]);

    println!("Press enter to stop and deallocate");
    let mut key = String::new();
    let _input = std::io::stdin().read_line(&mut key).unwrap();

    unlock_mem(ptr, amount_as_u8);
    drop(hog);
}
