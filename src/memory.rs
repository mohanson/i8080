pub trait Memory {
    fn get(&self, a: u16) -> u8;

    fn set(&mut self, a: u16, v: u8);

    fn get_word(&self, a: u16) -> u16 {
        u16::from(self.get(a)) | (u16::from(self.get(a + 1)) << 8)
    }

    fn set_word(&mut self, a: u16, v: u16) {
        self.set(a, (v & 0xFF) as u8);
        self.set(a + 1, (v >> 8) as u8)
    }
}

pub struct Linear {
    pub data: Vec<u8>,
}

impl Memory for Linear {
    fn get(&self, a: u16) -> u8 {
        self.data[usize::from(a)]
    }

    fn set(&mut self, a: u16, v: u8) {
        self.data[usize::from(a)] = v
    }
}

impl Linear {
    pub fn new() -> Self {
        Self { data: vec![0; 65536] }
    }
}
