use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MacAddress64(pub [u8; 8]);

impl MacAddress64 {
    pub fn new(bytes: [u8; 8]) -> Self {
        MacAddress64(bytes)
    }

    pub fn from_u64(value: u64) -> Self {
        MacAddress64(value.to_be_bytes())
    }
}

// Implement display formatting for MacAddress64
impl fmt::Display for MacAddress64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], self.0[6], self.0[7]
        )
    }
}
