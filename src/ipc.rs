use std::{
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
};

/*
 * type # bytes
 * health check # 1001
 * turn on lights # 1002
 * turn off lights # 1003
 * make noise # 1004
 * stop noise # 1005
 * start loud alarm # 1006
 * stop loud alarm # 1007
 * new event # 1010
 */
pub enum IpcMessage {
    MakeNoise,
}

impl IpcMessage {
    fn bytes(self) -> [u8; 4] {
        match self {
            Self::MakeNoise => [1, 0, 0, 4],
        }
    }
}

// printf "\x01\x00\x00\x05" | nc 127.0.0.1 8080
pub async fn send_alarm_notification(message: IpcMessage) -> std::io::Result<()> {
    let mut addrs_iter = "olympus:8080".to_socket_addrs()?;
    let addr = addrs_iter
        .next()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "address not found"))?;
    let mut stream = TcpStream::connect_timeout(&addr, std::time::Duration::from_secs(2))?;
    println!("connected to olympus");

    stream.write_all(&message.bytes())?;
    let mut buf = [0u8; 2];
    let _ = stream.read(&mut buf)?;
    if buf != "ok".as_bytes() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "response is not ok",
        ));
    }
    Ok(())
}
