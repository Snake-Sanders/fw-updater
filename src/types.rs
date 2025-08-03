#[derive(Debug, PartialEq)]
pub enum State {
    Init,      // waits for the configuration setup
    Setup,     // configured and ready for update
    Updating,  // processing incomming data
    Validated, // tx completed, data validated waiting to configm update
    Completed, // mark update pending  and restart
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Config = 0x01,
    Write = 0x02,
    Read = 0x03, // reserved
    Confirm = 0x04,
    Invalid = 0xFF,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Config {
    pub addr: u32,     // 32 bits address
    pub block_num: u8, // number of blocks, 256 - control fields
    pub crc: u32,      // image flash crc
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}
