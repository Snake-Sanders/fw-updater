// TODO:
// - add abort sequence
// - add mutex to the SPI in case other task need shared access to the Bus 

pub fn run() {
    // wait to receive the configuration: number of blocks, address, size, etc.

    // loop to receive the blocks and store them directly in flash

    // maybe send ACK the block was stored OK or Error

    // when blocks transmission finish check the CRC of the full file

    // expect a confirmation to mark update pending and reset
    // this way, several memory areas can be written before restarting.
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
