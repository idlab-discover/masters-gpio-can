wit_bindgen::generate!({
    path: "../../wit",
    generate_all
});

use std::time::Instant;
use wasi::can;
use wasi::can::types::{ErrorCode, Frame, FrameKind, Id};

const ROUNDS: usize = 1_000_000;

fn main() {
    if let Err(err) = run() {
        eprintln!("can failed: {err:?}");
    }
}

fn run() -> Result<(), ErrorCode> {
    let can = can::blocking::open("vcan")?;
    let standard_id = Id::Standard(1);
    let frame_kind = FrameKind::Data(vec![0x01]);
    let frame = Frame {
        id: standard_id,
        kind: frame_kind,
    };

    let mut vec = Vec::new();

    for _ in 0..ROUNDS {
        let t0 = Instant::now();
        can.transmit(&frame)?;
        let _ = can.receive()?;
        let elapsed = t0.elapsed();

        vec.push(elapsed.as_nanos());
    }

    let mut avg: u128 = vec.iter().sum();
    avg /= ROUNDS as u128;

    println!("avg: {}", avg);

    Ok(())
}
