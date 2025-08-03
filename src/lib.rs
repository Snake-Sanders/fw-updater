pub mod mock_spi_slave;
pub mod spi_slave;
pub mod types;

pub use mock_spi_slave::MockSpiSlave;
pub use spi_slave::{SpiError, SpiFrame, SpiSlave, BUS_SIZE, FRAME_DATA_SIZE};
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

    /// main entry point to fash the sw update
    pub fn run(&mut self) {
        let _ = self.block_read_setup();
        let _ = self.block_read_data();
        let _ = self.validate_received_data();
        let _ = self.block_read_confirmation();
    }

    /// wait to receive the configuration: number of blocks, address, size, etc.
    fn block_read_setup(&mut self) -> Result<(), SpiError> {
        self.read_bus().and_then(|frame| match frame.get_command() {
            Command::Config => {
                self.state = State::Setup;
                self.config = Config {
                    addr: frame.get_address(),
                    block_num: frame.get_block_num(),
                    crc: frame.get_crc(),
                };
                Ok(())
            }
            _invalid => Err(SpiError::BusError),
        })
    }

    /// "loop to receive the blocks and store them directly in flash"
    fn block_read_data(&mut self) -> Result<(), SpiError> {
        for i in 1..=self.config.block_num {
            match self.read_bus() {
                Ok(frame) => match frame.get_command() {
                    Command::Write => {
                        self.state = State::Updating;
                        let offset = self.calculate_offset(i);
                        self.write_update(&frame.data, offset);
                    }
                    _invalid => return Err(SpiError::BusError),
                },
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    fn validate_received_data(&mut self) -> Result<(), SpiError> {
        self.validate_flash_crc();
        todo!("when blocks transmission finish check the CRC of the full file");
    }

    /// expect a confirmation to mark update pending and reset
    fn block_read_confirmation(&mut self) -> Result<(), SpiError> {
        match self.read_bus() {
            Ok(frame) => match frame.get_command() {
                Command::Confirm => {
                    self.mark_update_pending();
                    self.system_reset();
                    Ok(())
                }

                _invalid => Err(SpiError::BusError),
            },
            _ => Err(SpiError::BusError),
        }
    }

    fn write_update(&self, _data: &[u8], offset: u32) {
        // TODO: call Bootloader write_update
        // use usize for offset
        dbg!("writing DFU offset: {}", offset);
    }

    fn system_reset(&self) {
        // TODO: call Bootloader system_reset()
    }

    fn mark_update_pending(&self) {
        // TODO: call Bootloader mark_update_pending()
    }

    fn calculate_offset(&self, index: u8) -> u32 {
        // memory address target offset
        self.config.addr + ((index as u32) * FRAME_DATA_SIZE as u32)
    }

    fn validate_flash_crc(&self) -> bool {
        // todo: the address and image size is in the config
        true
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
        data[5] = 0x04;
        // crc
        data[6] = 0x11;
        data[7] = 0x22;
        data[8] = 0x33;
        data[9] = 0x44;

        spi.set_bus_data(&data);

        let mut updater = Updater::new(&mut spi);

        let result = updater.block_read_setup();
        assert!(result.is_ok());

        let Config {
            addr,
            block_num,
            crc,
        } = updater.config;
        assert_eq!(addr, 0x78563412);
        assert_eq!(block_num, 4);
        assert_eq!(crc, 0x44332211);
    }

    #[test]
    fn updater_loops_reading_data_to_flash() {
        let mut spi = MockSpiSlave::new();

        let mut data = [0u8; BUS_SIZE];
        data[0] = Command::Write as u8;
        // blocks to write
        data[1] = 0x12;
        data[2] = 0x34;
        data[3] = 0x56;
        data[4] = 0x78;
        data[5] = 0x04;
        data[6] = 0x11;
        data[7] = 0x22;
        data[8] = 0x33;
        data[9] = 0x44;

        spi.set_bus_data(&data);

        let mut updater = Updater::new(&mut spi);
        updater.state = State::Setup;
        updater.config.block_num = 9;

        let result = updater.block_read_data();
        assert!(result.is_ok());
        assert_eq!(updater.state, State::Updating);
    }
}
