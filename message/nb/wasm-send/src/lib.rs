wit_bindgen::generate!({
    path: "../../../wit",
    world: "guest-nonblocking",
    generate_all,
});

use wasi::can::nonblocking::Error;
use wasi::can::types::{Frame, Id};

struct Component;

impl Guest for Component {
    fn run(connection: Can) {
        let standard_id = Id::Standard(1);
        let frame = Frame::new(
            standard_id,
            &[0x13, 0x37, 0xc0, 0xd3, 0x12, 0x34, 0x56, 0x78],
        )
        .unwrap();

        match connection.transmit(&frame) {
            Ok(None) => (),
            Ok(Some(frame)) => println!(
                "Dropped lower priority frame: id={:?}, data={:?}",
                frame.id(),
                frame.data()
            ),
            Err(Error::WouldBlock) => println!("Dropped sending frame"),
            Err(Error::Other(error)) => println!("{error}"),
        }
    }
}

export!(Component);
