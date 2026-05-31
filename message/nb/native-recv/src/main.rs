use embedded_can::{Frame, Id, nb::Can};
use socketcan::{CanSocket, Socket};
use std::time::Duration;

fn main() -> Result<(), socketcan::Error> {
    let mut can_socket = CanSocket::open("can0")?;
    can_socket.set_nonblocking(true)?;

    recv_loop(&mut can_socket)
}

fn recv_loop<C>(can: &mut C) -> Result<(), C::Error>
where
    C: Can,
{
    loop {
        let frame = match can.receive() {
            Ok(frame) => frame,
            Err(nb::Error::WouldBlock) => {
                std::thread::sleep(Duration::from_secs(3));
                continue;
            }
            Err(nb::Error::Other(err)) => return Err(err),
        };

        match frame.id() {
            Id::Standard(standard_id) => println!(
                "standard_id={}, data={:x?}",
                standard_id.as_raw(),
                frame.data()
            ),
            Id::Extended(extended_id) => println!(
                "extended_id={}, data={:x?}",
                extended_id.as_raw(),
                frame.data()
            ),
        }
    }
}
