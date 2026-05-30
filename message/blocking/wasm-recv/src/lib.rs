wit_bindgen::generate!({
    path: "../../../wit",
    world: "guest-blocking",
    generate_all,
});

struct Component;

use wasi::can::types::Id;

impl Guest for Component {
    fn run(connection: Can) {
        loop {
            let frame = connection.receive().unwrap();

            match frame.id() {
                Id::Standard(standard_id) => {
                    println!("standard_id: {:}, data: {:x?}", standard_id, frame.data())
                }
                Id::Extended(extended_id) => {
                    println!("extended_id: {:}, data: {:x?}", extended_id, frame.data())
                }
            }
        }
    }
}

export!(Component);
