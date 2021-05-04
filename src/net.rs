use tokio::net::UdpSocket;
use std::io;

pub async fn gethost() -> std::net::IpAddr {
    public_ip::addr().await.unwrap()
}

pub async fn connect(username: String) { // -> Result<&'static UdpSocket, io::Error> {

}