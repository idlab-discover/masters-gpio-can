wasmtime::component::bindgen!({
    world: "imports",
    with: {
        "wasi:can/blocking.can": crate::can::blocking::Can,
        "wasi:can/nonblocking.can": crate::can::nonblocking::Can,
    },
    imports: { default: trappable },
});
