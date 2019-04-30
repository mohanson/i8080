pub fn get(n: u8, b: usize) -> bool {
    (n & (1 << b)) != 0
}

pub fn set(n: u8, b: usize) -> u8 {
    n | (1 << b)
}

pub fn clr(n: u8, b: usize) -> u8 {
    n & !(1 << b)
}
