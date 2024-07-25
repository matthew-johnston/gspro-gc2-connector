use std::io::{self};
use std::{sync::mpsc::channel, thread, time::Duration};

use ball_event::BallEvent;
use clap::{command, Parser};
use log::{error, info};
use spin_sleep::native_sleep;

mod ball_event;
mod gs_pro;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    com_port: String,

    #[arg(long, default_value_t = 115_200)]
    com_baud: u32,

    #[arg(long)]
    gs_pro_ip: String,

    #[arg(long, default_value = "0921")]
    gs_pro_port: String,
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    let (sender, receiver) = channel::<BallEvent>();

    // Separate thread for the gspro connection. This will be used to send the ball events to the server.
    thread::spawn(move || {
        gs_pro::gspro_connect(&args.gs_pro_ip, &args.gs_pro_port, receiver);
    });

    let mut port = open_serial_port(&args.com_port, args.com_baud);

    let mut serial_buf: Vec<u8> = vec![0; 1000];
    info!(
        "Receiving data on {} at {} baud:",
        &args.com_port, &args.com_baud
    );

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
                error!("Failed to open \"{}\". Error: {}", port_name, e);
            }
        }

        // Sleep for a short duration to avoid high CPU usage
        native_sleep(Duration::from_millis(100));
    }
}
