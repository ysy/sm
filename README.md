## Serail Monitor for console mode
It's quit simple: receive bytes from seril port and write bytes to stdio, no convertion is invoked. Other serial monitors writeen in Rust that I've tried, all have utf-8 decoding issues. It's mainly used to recv debug message from SoC devices, in which case data corruption is inevitable.
