use std::io::{self};
use std::{sync::mpsc::channel, thread, time::Duration};

use clap::{command, Parser};
use log::{debug, error, info};
use spin_sleep::native_sleep;

mod ball_data;
mod data_line;
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
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp(None)
        .format_module_path(false)
        .init();

    let args = Args::parse();

    let (sender, receiver) = channel::<ball_data::BallData>();

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

    let mut accumulated_data = String::new();

    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                let data = String::from_utf8_lossy(&serial_buf[..t]);
                accumulated_data.push_str(&data);

                if let Some(pos) = accumulated_data.find('\n') {
                    let line_to_process = accumulated_data
                        .drain(..=pos)
                        .collect::<String>()
                        .trim_end_matches('\n')
                        .trim_end_matches('\r')
                        .to_string();

                    if line_to_process.starts_with("CT") {
                        debug!("Line to process {:?}", &line_to_process);

                        if let Some(data_line) = data_line::DataLine::from_line(&line_to_process) {
                            debug!("Parsed data line {:?}", &data_line);

                            let ball_event = ball_data::BallData::from(data_line);
                            debug!("Sending ball event {:?}", &ball_event);

                            sender.send(ball_event).unwrap();
                        }
                    }
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => error!("{:?}", e),
        }
    }
}

fn open_serial_port(port_name: &str, baud_rate: u32) -> Box<dyn serialport::SerialPort> {
    loop {
        info!(
            "Opening serial port \"{}\" at {} baud",
            port_name, baud_rate
        );

        match serialport::new(port_name, baud_rate).open() {
            Ok(p) => return p,
            Err(e) => {
                error!("Failed to open \"{}\". Error: {}", port_name, e);
            }
        }

        // Sleep for a short duration to avoid high CPU usage
        native_sleep(Duration::from_millis(100));
    }
}
