## Serail Monitor for console mode written in rust
It's quit simple: receive bytes from serial port and write bytes to stdio, no encoding convertion invoked. Other serial monitors writeen in Rust that I've tried all suffer from utf-8 decoding issues. It's mainly used to receive debug output from SoC devices, in which case data corruption is inevitable.
