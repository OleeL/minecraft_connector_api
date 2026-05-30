use minecraft_connector_api::buffer::{read_var_int, Buffer};

#[test]
fn writes_single_byte_var_int() {
    let mut buffer = Buffer::new();

    buffer.write_var_int(127);

    assert_eq!(buffer.0, vec![0x7f]);
}

#[test]
fn writes_multi_byte_var_int() {
    let mut buffer = Buffer::new();

    buffer.write_var_int(25565);

    assert_eq!(buffer.0, vec![0xdd, 0xc7, 0x01]);
}

#[test]
fn writes_big_endian_short() {
    let mut buffer = Buffer::new();

    buffer.write_be_short(25565);

    assert_eq!(buffer.0, vec![0x63, 0xdd]);
}

#[test]
fn writes_string_with_length_prefix() {
    let mut buffer = Buffer::new();

    buffer.write_string("mc", true);

    assert_eq!(buffer.0, vec![0x02, b'm', b'c']);
}

#[test]
fn writes_string_without_length_prefix() {
    let mut buffer = Buffer::new();

    buffer.write_string("mc", false);

    assert_eq!(buffer.0, vec![b'm', b'c']);
}

#[test]
fn reads_var_int_and_consumes_bytes() {
    let mut bytes = vec![0xdd, 0xc7, 0x01];

    let value = read_var_int(&mut bytes);

    assert_eq!(value, 25565);
    assert!(bytes.is_empty());
}
