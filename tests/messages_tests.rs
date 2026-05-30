use minecraft_connector_api::messages::{make_handshake_message, req_message};

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
