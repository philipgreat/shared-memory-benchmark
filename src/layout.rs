// src/layout.rs
#[repr(C)]
pub struct ShmLayout {
    /// 写入序号（递增）
    pub seq: u64,
    pub time: u64,
}
