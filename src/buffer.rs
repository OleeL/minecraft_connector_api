pub struct Buffer(pub Vec<u8>);

impl Buffer {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.0.push(byte)
    }

    pub fn write_be_short(&mut self, number: u16) {
        let bytes = number.to_be_bytes();
        self.0.extend(&bytes);
    }

    pub fn write_var_int(&mut self, mut number: usize) {
        for _ in 0..5 {
            if number & !0x7F == 0 {
                self.write_byte(number as u8);
                return;
            }
            self.write_byte((number & 0x7F | 0x80) as u8);
            number >>= 7;
        }
    }

    pub fn write_string(&mut self, message: &str, write_len: bool) {
        if write_len {
            self.write_var_int(message.len());
        }

        let bytes = message.as_bytes();
        self.0.extend(bytes);
    }
}

pub fn read_var_int(vec: &mut Vec<u8>) -> i32 {
    let mut res = 0i32;
    for i in 0..5 {
        let part = vec.remove(0);
        res |= (part as i32 & 0x7F) << (7 * i);
        if part & 0x80 == 0 {
            return res;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
