// src/shm.rs
use std::{
    ffi::CString,
    os::unix::io::RawFd,
    ptr,
};

use libc::*;

use crate::layout::ShmLayout;

const SHM_NAME: &str = "/shm_bench_test";
pub unsafe fn create_or_open() -> *mut ShmLayout {
    let name = CString::new(SHM_NAME).unwrap();
    let size = std::mem::size_of::<ShmLayout>();

    // 1. 尝试以不创建的方式打开
    let mut fd = shm_open(name.as_ptr(), O_RDWR, 0o666);
    
    if fd < 0 {
        // 2. 如果不存在，再尝试创建
        fd = shm_open(name.as_ptr(), O_CREAT | O_RDWR, 0o666);
        if fd < 0 { panic!("shm_open failed"); }
        
        // 3. 只有创建者才 truncate
        if ftruncate(fd, size as i64) != 0 {
            panic!("ftruncate failed");
        }
        println!("SHM created and truncated.");
    }

    let ptr = mmap(
        ptr::null_mut(),
        size,
        PROT_READ | PROT_WRITE,
        MAP_SHARED,
        fd,
        0,
    );

    if ptr == ( -1isize as *mut std::ffi::c_void ) { // MAP_FAILED
        panic!("mmap failed");
    }

    ptr as *mut ShmLayout
}
