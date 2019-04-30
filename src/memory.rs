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
