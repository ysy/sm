#![warn(rust_2018_idioms)]
use std::io::{self, Write, Read};
use serialport::{self, SerialPortType};
use clap::Parser;

/// serial monitor
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Serial port (COM20 or /dev/ttyUSB0)
    #[arg(short, long)]
    port: Option<String>,
    /// Baudrate
    #[arg(short, long, default_value_t=115200)]
    baud: u32,
    /// List serial port devices
    #[arg(short, long, default_value_t = false)]
    list: bool,
    /// Show timestamp
    #[arg(short, long, default_value_t = false)]
    time: bool,
}

fn list_devices() {
    let dev_list = serialport::available_ports().unwrap();
    let mut i = 0;
    for dev in dev_list {
        print!("{}: ", i);
        print!("[{}] ", dev.port_name);
        i += 1;
        match dev.port_type {
            SerialPortType::UsbPort(info) => {
                print!("USB {:04X}:{:04X}", info.vid, info.pid);
                print!(" {}", info.product.unwrap_or("".to_string()));
                print!(" {}", info.manufacturer.unwrap_or("".to_string()));
            },
            SerialPortType::PciPort => {
                print!("PCI");
            },
            SerialPortType::BluetoothPort=>{
                print!("Bluetooth");
            },
            SerialPortType::Unknown=>{
                print!("Unknown");
            }
        }
        print!("\n");
    }
}
fn main() -> io::Result<()> {
    let args =  Args::parse();

    if args.list {
        list_devices();
        return Ok(());
    }

    let port = args.port.expect("Serial port is not specified!");
    let mut seril_port = serialport::new(port.as_str(), args.baud)
                            .open()
                            .expect(
                                format!("Failed to open the serial port: {}\n",
                                     port.as_str()).as_str()) ;

    println!("Opened serial port: {} baudrate: {}", port.as_str(), args.baud);
    let mut buf = [0;1];
    let mut stdout = std::io::stdout().lock();
    loop {
        match seril_port.read(&mut buf) {
            Ok(_) => {
                if buf[0] == b'\n' {
                    // println!("##### new line detected");
                }
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