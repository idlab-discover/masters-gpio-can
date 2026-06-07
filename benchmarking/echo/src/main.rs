use embedded_can::blocking::Can;
use socketcan::{CanSocket, Socket};

fn main() -> Result<(), socketcan::Error> {
    let mut socket = CanSocket::open("vcan0")?;

    echo_loop(&mut socket)
}

fn echo_loop<C: Can>(can: &mut C) -> Result<(), C::Error> {
    loop {
        let frame = can.receive()?;
        can.transmit(&frame)?;
    }
}
