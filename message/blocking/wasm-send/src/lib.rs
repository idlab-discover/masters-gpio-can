wit_bindgen::generate!({
    path: "../../../wit",
    world: "guest-blocking",
    generate_all,
});

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

        connection.transmit(&frame).unwrap();
    }
}

export!(Component);
