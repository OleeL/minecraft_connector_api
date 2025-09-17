use crate::address::Address;
use crate::buffer::read_var_int;
use crate::buffer::Buffer;

pub async fn send_message_async(
    stream: &mut tokio::net::TcpStream,
    address: &Address,
) -> std::io::Result<Vec<u8>> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    // Handshake
    let (len, message) = make_handshake_message(&address.url, address.port);
    stream.write_all(&len).await?;
    stream.write_all(&message).await?;

    // Status request
    let (len2, message2) = req_message();
    stream.write_all(&len2).await?;
    stream.write_all(&message2).await?;

    // Read up to 5 bytes for VarInt length
    let mut header = vec![0u8; 5];
    stream.read_exact(&mut header).await?;
    let size = read_var_int(&mut header);

    // Remaining part of packet we care about is reported size minus 3 header bytes
    let left = size as usize - 3;

    let mut buffer = vec![0u8; left];
    stream.read_exact(&mut buffer).await?;
    Ok(buffer)
}

fn make_handshake_message(url: &str, port: u16) -> (Vec<u8>, Vec<u8>) {
    let mut buffer = Buffer::new();

    buffer.write_var_int(0x00); // Handshake packet id
    buffer.write_var_int(0xFF); // protocol version (legacy 255)

    buffer.write_string(url, true); // server address (with length)
    buffer.write_be_short(port); // server port
    buffer.write_var_int(1); // next state: status

    let mut len = Buffer::new();
    len.write_var_int(buffer.0.len());

    (len.0, buffer.0)
}

fn req_message() -> (Vec<u8>, Vec<u8>) {
    let mut buffer = Buffer::new();
    buffer.write_var_int(0x00); // status request

    let mut len = Buffer::new();
    len.write_var_int(buffer.0.len());

    (len.0, buffer.0)
}
