pub mod mock_spi_slave;
pub mod spi_slave;
pub mod types;

pub use mock_spi_slave::MockSpiSlave;
pub use spi_slave::{Command, SpiError, SpiFrame, SpiSlave, BUS_SIZE};
pub use types::*;

pub fn run<T: SpiSlave>(spi: &mut T) {
    let mut updater = Updater::new(spi);
    updater.run();
}

#[derive(Debug, PartialEq)]
struct Updater<'a, T: SpiSlave> {
    spi: &'a mut T,
    state: State,
    config: Config,
}

impl<'a, T: SpiSlave> Updater<'a, T> {
    pub fn new(spi: &'a mut T) -> Self {
        Updater {
            spi,
            state: State::Init,
            config: Config::new(),
        }
    }

    pub fn run(&mut self) {
        let _ = self.block_read_setup();
        let _ = self.block_read_data();
        let _ = self.validate_received_data();
        let _ = self.block_read_confirmation();
    }

    fn block_read_setup(&mut self) -> Result<(), SpiError> {
        // wait to receive the configuration: number of blocks, address, size, etc.
        let frame = self.read_bus()?;

        match frame.cmd {
            x if x == Command::Config as u8 => {
                self.state = State::Setup;
                let addr = frame.get_address();
                self.config = Config {
                    addr: addr,
                    block_num: 0,
                    crc: 0,
                };

                Ok(())
            }
            _invalid => Err(SpiError::BusError),
        }
    }

    fn block_read_data(&mut self) -> Result<(), SpiError> {
        // call `write_update` with each recieve data
        todo!("loop to receive the blocks and store them directly in flash");
    }

    fn validate_received_data(&mut self) -> Result<(), SpiError> {
        //
        todo!("when blocks transmission finish check the CRC of the full file");
    }
    fn block_read_confirmation(&mut self) -> Result<(), SpiError> {
        todo!("expect a confirmation to mark update pending and reset");
        // this way, several memory areas can be written before restarting.

        // call mark_update_pending()
        // call system_reset()
    }

    fn read_bus(&mut self) -> Result<SpiFrame, SpiError> {
        let mut buf = [0u8; BUS_SIZE];
        self.spi.read(&mut buf)?;

        let frame = SpiFrame {
            cmd: buf[0],
            data: buf[1..].try_into().unwrap(),
        };

        Ok(frame)
    }
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
        data[0] = Command::Config as u8;
        data[1] = 0xFA;
        spi.set_bus_data(&data);

        let mut updater = Updater::new(&mut spi);

        let result = updater.block_read_setup();
        assert!(result.is_ok());
        assert_eq!(updater.state, State::Setup);
    }

    #[test]
    // #[ignore = "not yet implemented"]
    fn updater_is_configured_via_spi_slave() {
        let mut spi = MockSpiSlave::new();

        let mut data = [0u8; BUS_SIZE];
        data[0] = Command::Config as u8;
        // address
        data[1] = 0x12;
        data[2] = 0x34;
        data[3] = 0x56;
        data[4] = 0x78;
        // num blocks
        data[5] = 0x03;
        // crc
        data[6] = 0x11;
        data[7] = 0x22;
        data[8] = 0x33;
        data[9] = 0x44;

        spi.set_bus_data(&data);

        let mut updater = Updater::new(&mut spi);

        let result = updater.block_read_setup();
        assert!(result.is_ok());

        let Config { addr, block_num, crc } = updater.config;
        assert_eq!(addr, 0x78563412);
        assert_eq!(block_num, 22);
        assert_eq!(crc, 33);
    }
}
