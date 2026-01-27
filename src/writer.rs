// src/writer.rs
use std::{sync::atomic::Ordering, time::Duration};
use crate::{shm::create_or_open, time::now_ns,layout::ShmLayout};

pub fn run_writer() {
    let shm = unsafe { &*create_or_open() };

    let mut seq = 0u64;
    
    loop {
        
        shm.time.store(now_ns(), Ordering::Relaxed);
        shm.seq.store(seq, Ordering::Release);
        
        seq += 1;
        if seq % 1_000_000_000 == 0 {
            println!("reaching {}" , seq);
        }
        //std::thread::sleep(Duration::from_nanos(1));;
    }
}
