pub use crate::Command;

pub trait SpiSlave {
    // reads from SPI bus into buf
    fn read(&mut self, buf: &mut [u8]) -> Result<(), SpiError>;
    // writes buf in SPI bus
    fn write(&mut self, buf: &[u8]) -> Result<(), SpiError>;
}

pub enum SpiError {
    BusError,
}

pub const BUS_SIZE: usize = 16;

#[derive(Debug, Clone, Copy)]
pub struct SpiFrame {
    pub cmd: u8,
    pub data: [u8; BUS_SIZE - 1], // 15 bytes (16 - 1 for cmd)
}

impl SpiFrame {
    pub fn get_command(&self) -> Command {
        match self.cmd {
            0x01 => Command::Config,
            0x02 => Command::Write,
            0x03 => Command::Read, // reserved
            0x04 => Command::Confirm,
            _ => Command::Invalid,
        }
    }

    pub fn get_address(&self) -> u32 {
        self.decode_u32(0)
    }

    pub fn get_block_num(&self) -> u8 {
        self.data[4]
    }

    pub fn get_crc(&self) -> u32 {
        self.decode_u32(5)
    }

    fn decode_u32(&self, i: usize) -> u32 {
        let mut n = 0;
        n |= self.data[i] as u32;
        n |= (self.data[i + 1] as u32) << 8;
        n |= (self.data[i + 2] as u32) << 16;
        n |= (self.data[i + 3] as u32) << 24;
        n
    }
}
