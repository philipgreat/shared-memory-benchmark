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
    let mut total = 0u64;
    let mut start = now_ns();
    let report_cycle = 1_000_000;
    let mut min_time = 1000000000000000u64;
    let mut max_time = 0u64;

    loop {

        let seq = shm.seq.load(Ordering::Acquire);
        let time = shm.time.load(Ordering::Relaxed);
        
        //total += time;

        // if(min_time>time){
        //     min_time = time;
        // }
        // if(max_time>time){
        //     max_time = time;
        // }
        
        if seq % report_cycle == 0 {
            let end = now_ns();
            println!("current seq {} and {} times timestamp @{}s and current lat {} ns", 
            seq ,report_cycle,(end - start)/1_000_000_000, end - time,
            );
            
           
            
        }

    }
}
