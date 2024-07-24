use std::io::{self, Write};
use std::time::Duration;

use clap::{command, Parser};

mod ball_event;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    port: String,

    #[arg(short, long, default_value_t = 115_200)]
    baud: u32,
}

fn main() {
    let args = Args::parse();
    let port_name = args.port;
    let baud_rate = args.baud;

    let port = serialport::new(&port_name, baud_rate)
        .timeout(Duration::from_millis(10_000))
        .open();

    match port {
        Ok(mut port) => {
            let mut serial_buf: Vec<u8> = vec![0; 1000];
            println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
            loop {
                match port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => {
                        let data = String::from_utf8_lossy(&serial_buf[..t]);
                        if data.starts_with("CT") {
                            let ball_event = ball_event::BallEvent::from_data_line(&data);
                            println!("{:?}", ball_event);
                        }

                        io::stdout().write_all(&serial_buf[..t]).unwrap();
                        io::stdout().flush().unwrap();
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
}
