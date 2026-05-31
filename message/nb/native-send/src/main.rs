use embedded_can::{Frame, StandardId, nb::Can};
use socketcan::{CanSocket, Socket};

fn main() -> Result<(), socketcan::Error> {
    let mut can_socket = CanSocket::open("can0")?;
    can_socket.set_nonblocking(true)?;

    send_once(&mut can_socket)
}

fn send_once<C>(can: &mut C) -> Result<(), C::Error>
where
    C: Can,
{
    let standard_id = StandardId::new(1).unwrap();
    let frame = C::Frame::new(
        standard_id,
        &[0x13, 0x37, 0xc0, 0xd3, 0x12, 0x34, 0x56, 0x78],
    )
    .unwrap();

    match can.transmit(&frame) {
        Ok(None) => Ok(()),
        Ok(Some(frame)) => {
            println!(
                "Dropped lower priority frame: id={:?}, data={:?}",
                frame.id(),
                frame.data()
            );
            Ok(())
        }
        Err(nb::Error::WouldBlock) => {
            println!("Dropped sending frame");
            Ok(())
        }
        Err(nb::Error::Other(err)) => Err(err),
    }
}
