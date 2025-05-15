use tokio::{
    io::{self, AsyncBufReadExt},
    net::UdpSocket,
    spawn,
};

use std::net::SocketAddr;

pub async fn listen(port: u16) -> anyhow::Result<()> {
    let socket = UdpSocket::bind(("0.0.0.0", port)).await?;
    println!("🛰️  Listening on port {}...", port);

    let mut buf = [0u8; 1024];
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        let msg = String::from_utf8_lossy(&buf[..len]);
        println!("\n[{}] {}", addr, msg.trim());
    }
}

pub async fn send_loop(target: &str, port: u16) -> anyhow::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    let target_addr: SocketAddr = format!("{}:{}", target, port).parse()?;

    println!("💬 Connected to {}. Type your message and hit enter.", target_addr);
    println!("🔚 Type `/exit` to quit.\n");

    let stdin = io::BufReader::new(io::stdin());
    let mut lines = stdin.lines();

    while let Some(Ok(line)) = lines.next_line().await {
        if line.trim() == "/exit" {
            println!("👋 Goodbye!");
            break;
        }

        socket.send_to(line.as_bytes(), &target_addr).await?;
    }

    Ok(())
}
