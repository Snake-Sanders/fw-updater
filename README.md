# fw-updater

Firmware updater over SPI

## Priorities

1. define a rough control flow for the update.
2. have a general SPI interface easy to inject as mock to be able to write
tests.
3. write the happy path first.
4. consider pending features and write down TODOS.

## TODOS

- Integrate function calls to the Bootloader.
- Make sure the size of the spi frame is not too big, otherwise writing the
incoming data to flash might take too long and miss the next incoming frame.
- Add error handling for clear error logs and reporting.
- User standarized error types for SPI instead of a general one.
- Consider that the SPI might be also used to access the Flash.

## Run

- Build with `cargo build`
- Run tests with `cargo test`

## Example

```rust
use fw_updater::{run, SpiSlave, SpiError};

let mut spi_config = spi::Config::default();
// configure HW specific SPI driver as slave
// set parameters to spi_config...frequency, phase, polarity...

// probably the SPI is also used to access the flash therefore it has to be
// shared.
let spi = Spi::new_blocking(p.SPI1, clk, mosi, miso, spi_config.clone());
// create instance implementing SpiSlave trait

let mut spi_slave = MySpiSlave::new(&mut spi);
run(&mut spi_slave);
```

## References

<https://docs.rs/embedded-hal/latest/embedded_hal/spi/trait.SpiBus.html>
<https://doc.rust-lang.org/book/ch15-05-interior-mutability.html>
