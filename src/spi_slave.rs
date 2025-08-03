pub trait SpiSlave {
    // reads from SPI bus into buf
    fn read(&mut self, buf: &mut [u8]) -> Result<(), SpiError>;
    // writes buf in SPI bus
    fn write(&mut self, buf: &[u8]) -> Result<(), SpiError>;
}
pub enum SpiError {
    BusError,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Config = 0x01,
    Write = 0x02,
    Read = 0x03,
    Confirm = 0x04,
}

pub const BUS_SIZE: usize = 16;

#[derive(Debug, Clone, Copy)]
pub struct SpiFrame {
    pub cmd: u8,
    pub data: [u8; BUS_SIZE - 1], // 15 bytes (16 - 1 for cmd)
}
