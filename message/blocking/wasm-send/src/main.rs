wit_bindgen::generate!({
    path: "../../../wit",
    generate_all,
});

use wasi::can::types::{ErrorCode, Frame, Id};

fn main() {
    if let Err(err) = run() {
        eprintln!("can failed: {err:?}");
    }
}

fn run() -> Result<(), ErrorCode> {
    let can = wasi::can::blocking::open()?;
    let standard_id = Id::Standard(1);
    let frame = Frame::new(
        standard_id,
        &[0x13, 0x37, 0xc0, 0xd3, 0x12, 0x34, 0x56, 0x78],
    )
    .unwrap();

    can.transmit(&frame)
}
