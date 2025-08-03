pub mod mock_spi_slave;
pub mod spi_slave;

pub use mock_spi_slave::MockSpiSlave;
pub use spi_slave::{SpiError, SpiFrame, SpiSlave};

const BUS_SIZE: usize = 16;

pub fn run<T: SpiSlave>(spi: &mut T) {
    let mut updater = Updater::new(spi);
    // wait to receive the configuration: number of blocks, address, size, etc.

    let _ = updater.wait_for_setup();
    // loop to receive the blocks and store them directly in flash

    // maybe send ACK the block was stored OK or Error

    // when blocks transmission finish check the CRC of the full file

    // expect a confirmation to mark update pending and reset
    // this way, several memory areas can be written before restarting.
}

// #[derive(Debug, PartialEq)]
struct Updater<'a, T: SpiSlave> {
    spi: &'a mut T,
    state: State,
}

impl<'a, T: SpiSlave> Updater<'a, T> {
    pub fn new(spi: &'a mut T) -> Self {
        Updater {
            spi,
            state: State::Init,
        }
    }
    pub fn wait_for_setup(&mut self) -> Result<(), SpiError> {
        let _frame = self.read_bus()?;
        self.state = State::Setup;
        Ok(())
    }

    fn read_bus(&mut self) -> Result<SpiFrame, SpiError> {
        let mut frame = [0u8; BUS_SIZE];
        self.spi.read(&mut frame)?;
        Ok(frame)
    }
}

#[derive(Debug, PartialEq)]
pub enum State {
    Init,      // waits for the configuration setup
    Setup,     // configured and ready for update
    Updating,  // processing incomming data
    Validated, // tx completed, data validated waiting to configm update
    Completed, // mark update pending  and restart
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_updater_starts_with_initialization_state() {
        let mut spi = MockSpiSlave::new();
        let updater = Updater::new(&mut spi);
        assert_eq!(updater.state, State::Init);
    }

    #[test]
    fn updater_reads_data_from_spi() {
        let mut spi = MockSpiSlave::new();

        let mut data = [0u8; BUS_SIZE];
        data[0] = 0x00;
        data[1] = 0xFA;
        spi.set_bus_data(&data);

        let mut updater = Updater::new(&mut spi);

        let result = updater.wait_for_setup();
        assert!(result.is_ok());
        assert_eq!(updater.state, State::Setup);
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn updater_is_configured_via_spi_slave() {}
}
