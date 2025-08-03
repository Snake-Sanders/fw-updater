use crate::spi_slave::{SpiSlave, SpiError};

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
