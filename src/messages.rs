use std::io::{prelude::*, Error};
use std::net::TcpStream;

use crate::address::Address;
use crate::buffer::read_var_int;
use crate::buffer::Buffer;

pub fn send_message(mut stream: &TcpStream, address: &Address) -> Result<Vec<u8>, Error> {
    let (len, message) = make_handshake_message(&address.url, address.port);
    stream.write(&len)?;
    stream.write(&message)?;

    let (len, message) = req_message();
    stream.write(&len)?;
    stream.write(&message)?;

    let mut buffer = vec![0; 5];
    let mut size_message = None;
    let mut left = usize::MAX;
    let mut second = false;

    while let Ok(_read) = stream.read_exact(&mut buffer) {
        if size_message.is_none() {
            let size = read_var_int(&mut buffer);
            size_message = Some(size);
            left = size as usize - 3;
        }

        if !second {
            second = true;
            buffer = vec![0; left];
        } else {
            break;
        }
    }
    Ok(buffer)
}

#[allow(dead_code)]
pub async fn send_message_async(
    stream: &mut tokio::net::TcpStream,
    _address: &Address,
) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    let mut buf = vec![0u8; 4096];
    let n = tokio::io::AsyncReadExt::read(stream, &mut buf).await?;
    buf.truncate(n);
    Ok(buf)
}

fn make_handshake_message(url: &str, port: u16) -> (Vec<u8>, Vec<u8>) {
    let mut buffer = Buffer::new();

    // Handshaking https://wiki.vg/Protocol#Handshake
    // Handshake in buffer
    buffer.write_var_int(0x00);

    buffer.write_var_int(0xFF); // 255

    // Writing length of url first then the url
    buffer.write_string(url, true);

    // Writes the port. Because the port is a number between 0 and 2^16
    // We need to send a short (16 bit number) and the buffer concatenates the
    // two binary numbers (The binary number is split in two) e.g. if you're
    // trying to send over 25565, it would be
    // [1100011 (99), 11011101 (221)] -> 110001111011101 (25565)
    buffer.write_be_short(port);

    // Write 1 for status (2 for login)
    buffer.write_var_int(1);

    // Creating a new array containing the length
    let mut len = Buffer::new();
    len.write_var_int(buffer.0.len());

    (len.0, buffer.0)
}

fn req_message() -> (Vec<u8>, Vec<u8>) {
    let mut buffer = Buffer::new();
    buffer.write_var_int(0x00);

    let mut len = Buffer::new();
    len.write_var_int(buffer.0.len());

    (len.0, buffer.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_status_request_packet() {
        let (len, message) = req_message();

        assert_eq!(len, vec![0x01]);
        assert_eq!(message, vec![0x00]);
    }

    #[test]
    fn builds_handshake_packet_for_default_port() {
        let (len, message) = make_handshake_message("localhost", 25565);

        assert_eq!(len, vec![0x10]);
        assert_eq!(
            message,
            vec![
                0x00, // packet id: handshake
                0xff, 0x01, // protocol version: 255
                0x09, // host length
                b'l', b'o', b'c', b'a', b'l', b'h', b'o', b's', b't', 0x63, 0xdd,
                // port: 25565
                0x01, // next state: status
            ]
        );
    }

    #[test]
    fn handshake_length_changes_with_hostname_length() {
        let (len, message) = make_handshake_message("mc", 25565);

        assert_eq!(len, vec![0x09]);
        assert_eq!(message.len(), 9);
    }
}
