use tokio::net::UdpSocket;
use std::net::SocketAddr;
use std::net;
use log::{self, info};
use std::io;

pub async fn gethost() -> std::net::IpAddr {
    public_ip::addr().await.unwrap()
}

pub async fn connect(_username: String, destination_ip: net::IpAddr) -> Result<(), io::Error> {
    let host = gethost().await;
    let addr = SocketAddr::new(host, 9192);

    let socketresult = UdpSocket::bind(addr).await;
    if let Err(error) = socketresult {
        Err(error)
    }

    let socket = socketresult.unwrap();
    info!("Socket bound as {}", addr.to_string());
    let connectresult = socket.connect(SocketAddr::new(destination_ip, 25565)).await;
    if let Err(error) = connectresult {
        Err(error)
    }

    Ok(())
}

pub async fn connect_from_str(username: &str, destination_ip: net::IpAddr) {
    connect(String::from(username), destination_ip).await;
}