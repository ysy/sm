#![warn(rust_2018_idioms)]

use std::io::{self, Write, Read};
use serialport::{self};

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyUSB0";
#[cfg(windows)]
const DEFAULT_TTY: &str = "COM20"; 

fn main() -> io::Result<()> {
    let mut seril_port = serialport::new("COM20", 115200).open().expect("failed to open the port");
    let mut buf = [0;1];
    let mut stdout = std::io::stdout().lock();
    loop {
        match seril_port.read(&mut buf) {
            Ok(n) => {
                // println!("{} bytes read", n);
                stdout.write_all(&mut buf)?;
            },

            Err(err) => {
                println!("Error occured when reading: {}", err);
                break;
            }
        }
    }
    
    Ok(())
}