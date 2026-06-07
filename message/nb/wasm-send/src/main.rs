wit_bindgen::generate!({
    path: "../../../wit",
    generate_all,
});

use wasi::can::nonblocking::Error;
use wasi::can::types::{ErrorCode, Frame, FrameKind, Id};

fn main() {
    if let Err(err) = run() {
        eprintln!("can failed: {err:?}");
    }
}

fn run() -> Result<(), ErrorCode> {
    let can = wasi::can::nonblocking::open("can")?;
    let standard_id = Id::Standard(1);
    let frame_kind = FrameKind::Data(vec![0x13, 0x37, 0xc0, 0xd3, 0x12, 0x34, 0x56, 0x78]);
    let frame = Frame {
        id: standard_id,
        kind: frame_kind,
    };

    match can.transmit(&frame) {
        Ok(None) => Ok(()),
        Ok(Some(frame)) => {
            println!(
                "Dropped lower priority frame: id={:?}, kind={:?}",
                frame.id, frame.kind
            );
            Ok(())
        }
        Err(Error::WouldBlock) => {
            println!("Dropped sending frame");
            Ok(())
        }
        Err(Error::Other(error)) => Err(error),
    }
}
