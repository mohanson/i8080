pub struct Memory {
    pub data: Vec<u8>,
}

impl i8080::Memory for Memory {
    fn get(&self, a: u16) -> u8 {
        self.data[usize::from(a)]
    }

    fn set(&mut self, a: u16, v: u8) {
        self.data[usize::from(a)] = v
    }
}

impl Memory {
    pub fn new() -> Self {
        Memory { data: vec![0; 65536] }
    }
}
