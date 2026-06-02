wasmtime::component::bindgen!({
    path: "../../../wit/deps/can",
    world: "imports",
    with: {
        "wasi:can/blocking.can": crate::can::blocking::Can,
        "wasi:can/nonblocking.can": crate::can::nonblocking::Can,
        "wasi:can/types.frame": crate::can::types::Frame,
    },
    imports: { default: trappable },
});
