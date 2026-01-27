// src/main.rs
mod shm;
mod layout;
mod writer;
mod reader;
mod time;
fn main() {
    let mode = std::env::args().nth(1)
        .expect("use: shm_bench [writer|reader]");

    match mode.as_str() {
        "writer" => writer::run_writer(),
        "reader" => reader::run_reader(),
        _ => panic!("unknown mode"),
    }
}

