use crate::spi_slave::{SpiError, SpiSlave, BUS_SIZE};
use std::cell::RefCell;

pub struct MockSpiSlave {
    bus: RefCell<[u8; BUS_SIZE]>,
}

impl Default for MockSpiSlave {
    fn default() -> Self {
        MockSpiSlave {
            bus: RefCell::new([0; BUS_SIZE]),
        }
    }
}

impl MockSpiSlave {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_bus_data(&self, data: &[u8; BUS_SIZE]) {
        self.bus.borrow_mut().copy_from_slice(data);
    }
}

impl SpiSlave for MockSpiSlave {
    fn read(&mut self, buf: &mut [u8]) -> Result<(), SpiError> {
        for (i, &byte) in self.bus.borrow().iter().enumerate() {
            buf[i] = byte;
        }
        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> Result<(), SpiError> {
        for (i, &byte) in buf.iter().enumerate() {
            self.bus.borrow_mut()[i] = byte;
        }

        Ok(())
    }
}
