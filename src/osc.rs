use std::{
    net::{SocketAddrV4, UdpSocket},
    str::FromStr,
    thread,
};

use anyhow::Result;
use rosc::OscPacket;

use crate::input::{self, InputHandler};

pub const DEFAULT_BASE_ADDR: &str = "/input";
pub const DEFAULT_ADDR: &str = "";

pub const DEFAULT_IP_ADDR: &str = "127.0.0.1";
pub const DEFAULT_RECEIVER_OSC_PORT: usize = 9000;
pub const DEFAULT_SENDER_OSC_PORT: usize = 9001;

#[derive(Debug)]
pub struct OscHandler {
    pub socket: UdpSocket,
}

pub static OSC_HANDLER: once_cell::sync::OnceCell<OscHandler> = once_cell::sync::OnceCell::new();

pub const UPDATE_LATENCY_DEFAULT: std::time::Duration = std::time::Duration::from_millis(100);

impl OscHandler {
    pub fn init_hadler() -> Result<()> {
        let sender_addr = SocketAddrV4::from_str(
            format!("{}:{}", DEFAULT_IP_ADDR, DEFAULT_SENDER_OSC_PORT).as_str(),
        )?;

        let handler = OscHandler {
            socket: UdpSocket::bind(sender_addr)?,
        };

        if let Err(e) = OSC_HANDLER.set(handler) {
            anyhow::bail!("failed to init osc handler: {:?}", e);
        } else {
            log::info!("Started recieving from {}", sender_addr);
        }

        Ok(())
    }

    pub fn get_handler() -> Result<&'static OscHandler> {
        OSC_HANDLER
            .get()
            .ok_or(anyhow::anyhow!("OSC Handler is not initialized"))
    }
}

pub fn start_osc() -> Result<()> {
    OscHandler::init_hadler()?;

    InputHandler::init()?;

    Ok(())
}

pub fn receive_packet(buf: &mut [u8]) -> Result<OscPacket> {
    let handler = OscHandler::get_handler()?;

    let socket = &handler.socket;

    match socket.recv_from(buf) {
        Ok((size, addr)) => {
            let (_buf, packet) = rosc::decoder::decode_udp(&buf[..size])?;
            log::info!("Received {:?} from {}", packet, addr);

            Ok(packet)
        }
        Err(e) => {
            log::error!("Error receiving from socket: {}", e);
            Err(anyhow::anyhow!("Error receiving from socket: {}", e))
        }
    }
}

pub fn send_packet(addr: &str, value: Vec<rosc::OscType>) -> Result<()> {
    let socket = &OscHandler::get_handler()?.socket;

    let receiver_addr = SocketAddrV4::from_str(
        format!("{}:{}", DEFAULT_IP_ADDR, DEFAULT_RECEIVER_OSC_PORT).as_str(),
    )?;

    let addr = format!("{}{}{}", DEFAULT_BASE_ADDR, DEFAULT_ADDR, addr);

    let packet = rosc::OscPacket::Message(rosc::OscMessage {
        addr: addr.to_string(),
        args: value,
    });

    let encoded_data = rosc::encoder::encode(&packet)?;

    socket.send_to(encoded_data.as_slice(), receiver_addr)?;
    log::info!("Sending {:?} to {}/{}", &packet, &receiver_addr, &addr);

    Ok(())
}
