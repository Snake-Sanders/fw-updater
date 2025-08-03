# fw-updater

Firmware updater over SPI

## Priorities

1. define a rough control flow for the update.
2. have a general SPI interface easy to inject as mock to be able to write
tests.
3. write the happy path first.
4. consider pending features and write down TODOS.

## References

https://docs.rs/embedded-hal/latest/embedded_hal/spi/trait.SpiBus.html
