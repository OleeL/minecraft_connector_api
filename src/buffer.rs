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
