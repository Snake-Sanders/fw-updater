pub trait SpiSlave {
    // reads from SPI bus into buf
    fn read(&mut self, buf: &mut [u8]) -> Result<(), SpiError>;
    // writes buf in SPI bus
    fn write(&mut self, buf: &[u8]) -> Result<(), SpiError>;
}
pub enum SpiError {
    BusError,
}

const BUS_SIZE: usize = 16;

pub type SpiFrame = [u8; BUS_SIZE];

}
