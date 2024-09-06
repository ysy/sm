#![warn(rust_2018_idioms)]
use std::io::{self, Write, Read};
use std::str;
use serialport::{self, SerialPortType};
use clap::Parser;
use chrono::prelude::*;

/// serial monitor
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Serial port (COM20 or /dev/ttyUSB0)
    #[arg(short, long)]
    port: Option<String>,
    /// Baudrate
    #[arg(short, long, default_value_t=String::from("115200"))]
    baud: String,
    /// List serial port devices
    #[arg(short, long, default_value_t = false)]
    list: bool,
    /// Show timestamp
    #[arg(short, long, default_value_t = false)]
    time: bool,
    /// write log file
    #[arg(short, long, default_value_t = false)]
    write_log_file: bool,
}
#[cfg(target_os = "windows")]
fn add_com_prefix(path: &str) -> String {
    if !path.to_lowercase().starts_with("com") {
        format!("COM{}", path)
    } else {
        path.to_string()
    }
}

#[cfg(not(target_os = "windows"))]
fn add_com_prefix(path: &str) -> String {
    path.to_string()
}

fn parse_baud_rate(baud_rate: &str) -> Result<u32, String> {
    let trimmed = baud_rate.trim().to_lowercase();

    if trimmed.ends_with('m') {
        let value = trimmed.trim_end_matches('m');
        match value.parse::<f64>() {
            Ok(num) => Ok((num * 1_000_000.0) as u32),
            Err(_) => Err(format!("Invalid baud rate format: {}", baud_rate))
        }
    } else {
        match trimmed.parse::<u32>() {
            Ok(num) => Ok(num),
            Err(_) => Err(format!("Invalid baud rate format: {}", baud_rate))
        }
    }
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

fn get_timestamp() ->String {
    let local: DateTime<Local> = Local::now();
    format!("{}", local.format("%m-%d %H:%M:%S%.3f "))
}

fn get_log_file_name() -> String {
    let local: DateTime<Local> = Local::now();
    format!("{}", local.format("Serial_Log_%Y-%m-%d_%H_%M_%S.log"))
}

fn main() -> io::Result<()> {
    let args =  Args::parse();

    if args.list {
        list_devices();
        return Ok(());
    }


    let port = args.port.expect("Serial port is not specified!");
    let port = add_com_prefix(&port);


    let baud = parse_baud_rate(&args.baud).expect("Invalid baud rate!");
    let mut seril_port = serialport::new(port.as_str(), baud)
                            .open()
                            .expect(
                                format!("Failed to open the serial port: {}\n",
                                     port.as_str()).as_str());

    println!("Opened serial port: {} baudrate: {}", port.as_str(), args.baud);
    let mut buf = [0;1];
    let mut stdout = std::io::stdout().lock();
    let mut new_line_detected = true;

    let mut log_file = None;
    if args.write_log_file {
        log_file = std::fs::File::create(get_log_file_name()).ok();
    }

    loop {
        match seril_port.read(&mut buf) {
            Ok(_) => {
                if buf[0] == b'\n' || buf[0] == b'\r' {
                    new_line_detected = true;
                } else if new_line_detected {
                    if args.time {
                        let tm = get_timestamp();
                        stdout.write_all(tm.as_bytes()).unwrap_or(());

                        if args.write_log_file {
                            log_file.as_mut().unwrap().write_all(tm.as_bytes()).unwrap_or(());
                        }
                    }
                    new_line_detected = false;
                }
                // stdout.write_all(&mut buf).unwrap_or(());
                if args.write_log_file {
                    log_file.as_mut().unwrap().write_all(&mut buf).unwrap_or(());
                }

                match str::from_utf8(&buf){
                    Ok(_) => {},
                    Err(_) => {
                        // println!("#ERR: {}", err);
                        print!("*");
                        continue;
                    }
                }

                match stdout.write_all(&mut buf) {
                    Ok(_) => {},
                    Err(err) => {
                        println!("#ERR: {}", err);
                    }
                }
            },

            Err(err) => {
                println!("Error occured when reading: {}", err);
                break;
            }
        }
    }

    Ok(())
}
