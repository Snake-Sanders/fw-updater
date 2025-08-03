pub enum SpiError {
    BusError,
}

pub trait SpiSlave {
    // reads from SPI bus into buf
    fn read(&mut self, buf: &mut [u8]) -> Result<(), SpiError>;
    // writes buf in SPI bus
    fn write(&mut self, buf: &[u8]) -> Result<(), SpiError>;
}

pub struct MockSpiSlave {
    bus: [u8; 256],
}
impl Default for MockSpiSlave {
    fn default() -> Self {
        MockSpiSlave { bus: [0u8; 256] }
    }
}

impl MockSpiSlave {
    pub fn new() -> Self {
        Self::default()
    }
}

impl SpiSlave for MockSpiSlave {
    fn read(&mut self, buf: &mut [u8]) -> Result<(), SpiError> {
        for (i, &byte) in self.bus.iter().enumerate() {
            buf[i] = byte;
        }
        Ok(())
    }
    fn write(&mut self, buf: &[u8]) -> Result<(), SpiError> {
        for (i, &byte) in buf.iter().enumerate() {
            self.bus[i] = byte;
        }

        Ok(())
    }
}
