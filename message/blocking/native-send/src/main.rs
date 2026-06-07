use embedded_can::{Frame, StandardId, blocking::Can};
use socketcan::{CanSocket, Socket};

fn main() -> Result<(), socketcan::Error> {
    let mut can_socket = CanSocket::open("can0")?;

    send_once(&mut can_socket)
}

fn send_once<C: Can>(can: &mut C) -> Result<(), C::Error> {
    let standard_id = StandardId::new(1).unwrap();
    let frame = C::Frame::new(
        standard_id,
        &[0x13, 0x37, 0xc0, 0xd3, 0x12, 0x34, 0x56, 0x78],
    )
    .unwrap();

    can.transmit(&frame)
}
