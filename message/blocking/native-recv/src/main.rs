use embedded_can::{Frame, Id, blocking::Can};
use socketcan::{CanSocket, Socket};

fn main() -> Result<(), socketcan::Error> {
    let mut can_socket = CanSocket::open("can0")?;

    recv_loop(&mut can_socket)
}

fn recv_loop<C>(can: &mut C) -> Result<(), C::Error>
where
    C: Can,
{
    loop {
        let frame = can.receive()?;

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
