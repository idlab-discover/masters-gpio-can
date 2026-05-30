wit_bindgen::generate!({
    path: "../../../wit",
    world: "guest-blocking",
    generate_all,
});

use wasi::can::types::{Frame, Id};

struct Component;

impl Guest for Component {
    fn run(connection: Can) {
        let frame = Frame::new(Id::Standard(1), &[0x13, 0x37]).unwrap();
        connection.transmit(&frame).unwrap();
    }
}

export!(Component);
