wit_bindgen::generate!({
    path: "../../../wit",
    generate_all,
});

use wasi::can::nonblocking::Error;
use wasi::can::types::{ErrorCode, Id};

fn main() {
    if let Err(err) = run() {
        eprintln!("can failed: {err:?}");
    }
}

fn run() -> Result<(), ErrorCode> {
    let can = wasi::can::nonblocking::open()?;
    loop {
        let frame = match can.receive() {
            Ok(frame) => frame,
            Err(Error::WouldBlock) => {
                std::thread::sleep(std::time::Duration::from_secs(3));
                continue;
            }
            Err(Error::Other(error)) => return Err(error),
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
