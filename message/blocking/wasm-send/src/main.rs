wit_bindgen::generate!({
    path: "../../../wit",
    generate_all,
});

use wasi::can::types::{ErrorCode, Frame, FrameKind, Id};

fn main() {
    if let Err(err) = run() {
        eprintln!("can failed: {err:?}");
    }
}

fn run() -> Result<(), ErrorCode> {
    let can = wasi::can::blocking::open("can")?;
    let standard_id = Id::Standard(1);
    let frame_kind = FrameKind::Data(vec![0x13, 0x37, 0xc0, 0xd3, 0x12, 0x34, 0x56, 0x78]);
    let frame = Frame {
        id: standard_id,
        kind: frame_kind,
    };

    can.transmit(&frame)
}
