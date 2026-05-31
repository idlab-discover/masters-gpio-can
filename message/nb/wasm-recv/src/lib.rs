wit_bindgen::generate!({
    path: "../../../wit",
    world: "guest-nonblocking",
    generate_all,
});

use wasi::can::nonblocking::Error;
use wasi::can::types::Id;

struct Component;

impl Guest for Component {
    fn run(connection: Can) {
        loop {
            let frame = match connection.receive() {
                Ok(frame) => frame,
                Err(Error::WouldBlock) => {
                    std::thread::sleep(std::time::Duration::from_secs(3));
                    continue;
                }
                Err(Error::Other(error)) => {
                    println!("{error}");
                    return;
                }
            };

            match frame.id() {
                Id::Standard(standard_id) => {
                    println!("standard_id={}, data={:x?}", standard_id, frame.data())
                }
                Id::Extended(extended_id) => {
                    println!("extended_id={}, data={:x?}", extended_id, frame.data())
                }
            }
        }
    }
}

export!(Component);
