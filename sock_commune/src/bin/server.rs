use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:12345")?;
    // Buffer of 10 zeros
    let mut buf = [0; 10];
    socket.recv_from(&mut buf)?;

    let msg = String::from_utf8(buf.to_vec()).unwrap();
    println!("{}", msg);
    Ok(())
}
