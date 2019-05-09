/// Port-mapped I/O (PMIO)
pub trait Device {
    fn pmi(&mut self, a: u8);
    fn pmo(&self) -> u8;
}
