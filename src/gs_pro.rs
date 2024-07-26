// https://gsprogolf.com/GSProConnectV1.html

use std::{
    io::{self, Read, Write},
    net::TcpStream,
    sync::mpsc::Receiver,
    thread,
    time::Duration,
};

use log::{error, info};
use spin_sleep::native_sleep;

use crate::ball_data::BallData;

// Function to handle the connection to the server
pub fn gspro_connect(ip: &str, port: &str, receiver: Receiver<BallData>) {
    let address = format!("{}:{}", ip, port);

    // Keep attempting to connect to the server
    loop {
        handle_connect(&address, &receiver);

        // Sleep for a short duration to avoid high CPU usage
        native_sleep(Duration::from_secs(5));
    }
}

fn handle_connect(address: &String, receiver: &Receiver<BallData>) {
    // Attempt to connect to the server
    match TcpStream::connect(address) {
        Ok(mut stream) => {
            info!("Successfully connected to server {}", address);

            // Set the stream to non-blocking mode to handle read/write operations properly
            stream
                .set_nonblocking(true)
                .expect("Failed to initiate non-blocking");

            // Spawn a thread to handle reading from the stream
            let read_stream = stream.try_clone().expect("Failed to clone stream");
            let read_thread_handle = thread::spawn(move || {
                handle_read(read_stream);
            });

            let mut last_ball_data_reading: Option<BallData> = None;

            // Main loop to handle writing to the stream
            loop {
                // Check if the read thread has finished, if so, break the loop
                if read_thread_handle.is_finished() {
                    break;
                }

                if let Ok(ball_event) = receiver.try_recv() {
                    // Check if the ball event is the same as the last one, if so, skip it
                    if let Some(ref last_ball_data) = last_ball_data_reading {
                        if last_ball_data.ShotNumber == ball_event.ShotNumber {
                            continue;
                        }
                    }

                    last_ball_data_reading = Some(ball_event.clone());

                    // Convert the ball event to a json string to send to the server
                    if let Ok(ball_json) = serde_json::to_string(&ball_event) {
                        info!("Sending: {}", ball_json);

                        if let Err(e) = stream.write_all(ball_json.as_bytes()) {
                            error!("Failed to write to server: {}", e);
                            //break;
                        }
                    }
                }

                // Sleep for a short duration to avoid high CPU usage
                native_sleep(Duration::from_millis(100));
            }
        }
        Err(e) => {
            error!("Failed to connect: {}", e);
        }
    }
}

// Function to handle reading from the stream
fn handle_read(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    loop {
        match stream.read(&mut buf) {
            Ok(0) => {
                // Connection was closed
                info!("Connection closed by server");
                break;
            }
            Ok(t) => {
                // Handle the data received from the server
                let data = String::from_utf8_lossy(&buf[..t]);
                info!("Received: {}", data);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // No data available yet, so wait for a short duration
                thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                error!("Failed to read from server: {}", e);
                break;
            }
        }
    }
}
