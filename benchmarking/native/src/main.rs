use embedded_can::{Frame, StandardId, blocking::Can};
use socketcan::{CanSocket, Socket};
use std::time::Instant;

const ROUNDS: usize = 1_000_000;

fn main() -> Result<(), socketcan::Error> {
    let mut socket = CanSocket::open("vcan0")?;

    send_multiple(&mut socket)
}

fn send_multiple<C: Can>(can: &mut C) -> Result<(), C::Error> {
    let standard_id = StandardId::new(1).unwrap();
    let frame = C::Frame::new(standard_id, &[0x01]).unwrap();

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
