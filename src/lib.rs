pub mod mock_spi_slave;
pub mod spi_slave;

pub use mock_spi_slave::MockSpiSlave;
pub use spi_slave::{SpiError, SpiFrame, SpiSlave};

const BUS_SIZE: usize = 16;

pub fn run<T: SpiSlave>(spi: &mut T) {
    let mut updater = Updater::new(spi);

    let _ = updater.block_read_setup();
    let _ = updater.block_read_data();
    let _ = updater.validate_received_data();
    let _ = updater.block_read_confirmation();
}

#[derive(Debug, PartialEq)]
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

    pub fn block_read_setup(&mut self) -> Result<(), SpiError> {
        // wait to receive the configuration: number of blocks, address, size, etc.
        let frame = self.read_bus()?;

        self.state = State::Setup;
        Ok(())
    }

    pub fn block_read_data(&mut self) -> Result<(), SpiError> {
        // call `write_update` with each recieve data
        todo!("loop to receive the blocks and store them directly in flash");
    }

    pub fn validate_received_data(&mut self) -> Result<(), SpiError> {
        //
        todo!("when blocks transmission finish check the CRC of the full file");
    }
    pub fn block_read_confirmation(&mut self) -> Result<(), SpiError> {
        todo!("expect a confirmation to mark update pending and reset");
        // this way, several memory areas can be written before restarting.

        // call mark_update_pending()
        // call system_reset()
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

#[derive(Debug, PartialEq)]
pub enum Commands {
    Config = 0x01,
    Write = 0x02,
    Read = 0x03,
    Confirm = 0x04,
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

        let result = updater.block_read_setup();
        assert!(result.is_ok());
        assert_eq!(updater.state, State::Setup);
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn updater_is_configured_via_spi_slave() {}
}
