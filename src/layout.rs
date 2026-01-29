// src/layout.rs
use std::sync::atomic::{AtomicU64, AtomicU32};

#[repr(C)]
pub struct ShmLayout {
    /// 写入序号（递增）
    pub seq: AtomicU64,
    pub time: AtomicU64,
}
