// src/reader.rs
use std::sync::atomic::Ordering;
use crate::shm::create_or_open;
use crate::time::now_ns;
pub fn run_reader() {
    let shm = unsafe { &*create_or_open() };

    // let mut last_seq = 0;
    // let mut counter = 0u64;
    // let mut right_counter =0u64; 
    // let mut wrong_counter =0u64; 
     
    loop {
        let seq = shm.seq.load(Ordering::Relaxed);
        
        if seq % 1000 == 0 {
            println!("found seq {} ", seq);
        }

    }
}
