wasmtime::component::bindgen!({
    path: "wit",
    world: "imports",
    with: {
        // "wasi:io/poll.pollable": wasmtime_wasi_io::poll::DynPollable,
        "wasi:gpio/digital.output-pin": crate::gpio::digital::OutputPin,
        "wasi:gpio/digital.stateful-output-pin": crate::gpio::digital::StatefulOutputPin,
        "wasi:gpio/digital.input-pin": crate::gpio::digital::InputPin,
        // "wasi:gpio/digital-async.input-wait-pin": crate::gpio::digital_async::InputWaitPin,
        "wasi:gpio/pwm.pwm-pin": crate::gpio::pwm::PwmPin,
    },
    imports: { default: trappable },
});
