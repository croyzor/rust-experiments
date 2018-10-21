use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:12346")?;
    socket.connect("127.0.0.1:12345")?;
    let msg = "hello".as_bytes();
    socket.send(msg)?;
    Ok(())
}
