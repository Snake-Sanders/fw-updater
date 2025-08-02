pub fn run() {
    let _updater = Updater::new();
    // wait to receive the configuration: number of blocks, address, size, etc.

    // loop to receive the blocks and store them directly in flash

    // maybe send ACK the block was stored OK or Error

    // when blocks transmission finish check the CRC of the full file

    // expect a confirmation to mark update pending and reset
    // this way, several memory areas can be written before restarting.
}

// #[derive(Debug, PartialEq)]
struct Updater{
   state: State,
}

impl Updater {
    pub fn new() -> Updater {
        Updater{
            state: State::Init
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum State{
    Init, // waits for the configuration setup
    Setup, // configured and ready for update
    Updating, // processing incomming data
    Validated, // tx completed, data validated waiting to configm update
    Completed // mark update pending  and restart
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_updater_starts_with_initialization_state(){
        let updater = Updater::new();
        assert_eq!(updater.state , State::Init) ;
    }
}
