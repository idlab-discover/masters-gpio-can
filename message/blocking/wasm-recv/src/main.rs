wit_bindgen::generate!({
    path: "../../../wit",
    generate_all,
});

use wasi::can::types::{ErrorCode, Id};

fn main() {
    if let Err(err) = run() {
        eprintln!("can failed: {err:?}");
    }
}

fn run() -> Result<(), ErrorCode> {
    let can = wasi::can::blocking::open("can")?;
    loop {
        let frame = can.receive()?;

        match frame.id {
            Id::Standard(standard_id) => {
                println!("standard_id={}, kind={:?}", standard_id, frame.kind)
            }
            Id::Extended(extended_id) => {
                println!("extended_id={}, kind={:?}", extended_id, frame.kind)
            }
        }
    }
}
