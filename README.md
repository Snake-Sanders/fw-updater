# fw-updater

Firmware updater over SPI

## Priorities

1. define a rough control flow for the update.
2. have a general SPI interface easy to inject as mock to be able to write
tests.
3. write the happy path first.
4. consider pending features and write down TODOS.

## TODOS

- Make sure the size of the spi frame is not too big, otherwise writing the
incoming data to flash might take too long and miss the next incoming frame.
- User standarized error types for SPI instead of a general one.

## References

<https://docs.rs/embedded-hal/latest/embedded_hal/spi/trait.SpiBus.html>
https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
