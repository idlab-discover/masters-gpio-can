use embedded_can::{Frame, StandardId, blocking::Can};
use socketcan::{CanSocket, Socket};

fn main() -> Result<(), socketcan::Error> {
    let mut can_socket = CanSocket::open("can0")?;
    let id = 1;
    let data = &[0x13, 0x37, 0xc0, 0xd3, 0x12, 0x34, 0x56, 0x78];

    send_once(&mut can_socket, id, data)
}

fn send_once<C>(can: &mut C, id: u16, data: &[u8]) -> Result<(), C::Error>
where
    C: Can,
{
    let standard_id = StandardId::new(id).unwrap();
    let frame = C::Frame::new(standard_id, data).unwrap();

    can.transmit(&frame)
}
