use std::net::UdpSocket;
use crate::time::now_ns;

pub fn run_reader() {
    // 1️⃣ 绑定接收端口
    let socket = UdpSocket::bind("0.0.0.0:9000")
        .expect("bind udp receiver failed");

    // 可选：扩大内核接收缓冲，减少丢包
    //let _ = socket.set_recv_buffer_size(4 * 1024 * 1024);

    let mut buf = [0u8; 16];

    let mut start = now_ns();
    let report_cycle = 1_000_000u64;

    

    loop {
        // 2️⃣ 接收 UDP
        let (n, _) = socket.recv_from(&mut buf).expect("recv failed");
        if n != 16 {
            continue;
        }

        // 3️⃣ 解析 seq / time（网络字节序）
        let seq = u64::from_be_bytes(buf[..8].try_into().unwrap());
        let time = u64::from_be_bytes(buf[8..].try_into().unwrap());

        

        

        // 5️⃣ 周期性打印
        if seq % report_cycle == 0 {

            let end = now_ns();
            let lat = end - time;

            println!(
                "current seq {} | elapsed {} s | lat {} ns ",
                seq,
                (end - start) / 1_000_000_000,
                lat
            );

            // 重置窗口
           
        }
    }
}
