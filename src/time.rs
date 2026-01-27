pub fn now_ns() -> u64 {    
    get_ticks() 
}

#[inline(always)]
fn get_ticks() -> u64 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        // Load fence: prevents the CPU from executing rdtsc
        // before previous instructions have finished.
        _mm_lfence();
        let res = _rdtsc();
        _mm_lfence();
        res
    }

    #[cfg(target_arch = "aarch64")]
    {
        let val: u64;
        unsafe {
            // ARM64 system counter is usually already synchronized
            std::arch::asm!("mrs {}, cntvct_el0", out(reg) val);
        }
        val
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
    0
}
