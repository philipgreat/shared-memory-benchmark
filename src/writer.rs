// src/writer.rs
use std::{net::UdpSocket, sync::atomic::Ordering};
use crate::{layout::ShmLayout, shm::create_or_open, time::now_ns};

pub fn run_writer() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("bind failed");
    socket
        .connect("127.0.0.1:9000")
        .expect("connect failed");

    let mut seq: u64 = 0;
    let mut buf = [0u8; 16];

    loop {
        let msg = ShmLayout {
            seq,
            time: now_ns(),
        };

        // 2️⃣ 手动序列化（网络字节序，大端）
        buf[..8].copy_from_slice(&msg.seq.to_be_bytes());
        buf[8..].copy_from_slice(&msg.time.to_be_bytes());

        // 3️⃣ 发送
        socket.send(&buf).unwrap();

        seq += 1;


    }

}
