pub mod spi_slave;

pub use spi_slave::{MockSpiSlave, SpiSlave};

pub fn run<T: SpiSlave>(spi: &T) {
    let _updater = Updater::new(spi);
    // wait to receive the configuration: number of blocks, address, size, etc.

    // loop to receive the blocks and store them directly in flash

    // maybe send ACK the block was stored OK or Error

    // when blocks transmission finish check the CRC of the full file

    // expect a confirmation to mark update pending and reset
    // this way, several memory areas can be written before restarting.
}

// #[derive(Debug, PartialEq)]
struct Updater<'a, T: SpiSlave> {
    spi: &'a T,
    state: State,
}

impl<'a, T: SpiSlave> Updater<'a, T> {
    pub fn new(spi: &'a T) -> Self {
        Updater {
            spi,
            state: State::Init,
        }
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
        let spi = MockSpiSlave::new();
        let updater = Updater::new(&spi);

        assert_eq!(updater.state, State::Init);
    }

    #[test]
    fn updater_reads_data_from_spi() {
        let mut spi = MockSpiSlave::new();
        let updater = Updater::new(&spi);

        let data = [0x00, 0xFA];
        spi.write(&data);
        assert_eq!(updater.state, State::Setup)
    }

    #[test]
    fn updater_is_configured_via_spi_slave() {}
}
