wit_bindgen::generate!({
    path: "../../wit",
    generate_all
});

use wasi::can::types::{ErrorCode, Id};
use wasi::can;
use std::time::Instant;

const ROUNDS: usize = 1_000_000;

fn main() {
    if let Err(err) = run() {
        eprintln!("can failed: {err:?}");
    }
}

fn run() -> Result<(), ErrorCode> {
    let can = can::blocking::open("vcan")?;
    let standard_id = Id::Standard(1);
    let frame = can.new_frame(standard_id, &[0x01]).unwrap();

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
