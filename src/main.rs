use std::io::{self, Write};

use std::{sync::mpsc::channel, thread, time::Duration};

use ball_event::BallEvent;
use clap::{command, Parser};
use log::{error, info};

mod ball_event;
mod gs_pro;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    com_port: String,

    #[arg(short, long, default_value_t = 115_200)]
    com_baud: u32,

    #[arg(short, long)]
    gs_pro_ip: String,

    #[arg(short, long, default_value = "0921")]
    gs_pro_port: String,
}

fn main() {
    env_logger::init();

    let args = Args::parse();
    let port_name = args.com_port;
    let baud_rate = args.com_baud;

    let (sender, receiver) = channel::<BallEvent>();

    // Separate thread for the gspro connection. This will be used to send the ball events to the server.
    thread::spawn(move || {
        gs_pro::gspro_connect(&args.gs_pro_ip, &args.gs_pro_port, receiver);
    });

    let mut port = open_serial_port(&port_name, baud_rate);

    let mut serial_buf: Vec<u8> = vec![0; 1000];
    info!("Receiving data on {} at {} baud:", &port_name, &baud_rate);

    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                let data = String::from_utf8_lossy(&serial_buf[..t]);
                if data.starts_with("CT") {
                    let ball_event = ball_event::BallEvent::from_data_line(&data);
                    if let Some(ball_event) = ball_event {
                        sender.send(ball_event).unwrap();
                    }
                }

                info!("{}", data);

                // io::stdout().write_all(&serial_buf[..t]).unwrap();
                // io::stdout().flush().unwrap();
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => error!("{:?}", e),
        }
    }
}

fn open_serial_port(port_name: &str, baud_rate: u32) -> Box<dyn serialport::SerialPort> {
    loop {
        match serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(10))
            .stop_bits(serialport::StopBits::One)
            .parity(serialport::Parity::None)
            .data_bits(serialport::DataBits::Eight)
            .open()
        {
            Ok(p) => return p,
            Err(e) => {
                // TODO: Prevent log spam if the port is not available (or incorrect port)

                error!("Failed to open \"{}\". Error: {}", port_name, e);
                // ::std::process::exit(1);
            }
        }
    }
}
