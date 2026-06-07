wit_bindgen::generate!({
    path: "../../../wit",
    generate_all,
});

use embedded_can::blocking::Can as BlockingCan;
use embedded_can::nb::Can as NonblockingCan;
use embedded_can::{
    Error as CanError, ErrorKind as CanErrorKind, ExtendedId, Frame, Id, StandardId,
};
use embedded_hal::digital::{
    Error as DigitalError, ErrorKind as DigitalErrorKind, ErrorType as DigitalErrorType, InputPin,
    OutputPin, StatefulOutputPin,
};
use embedded_hal::pwm::{
    Error as PwmError, ErrorKind as PwmErrorKind, ErrorType as PwmErrorType, SetDutyCycle,
};
use nb::Error as NonblockingError;

use wasi::can::blocking::Can as WasiBlockingCan;
use wasi::can::nonblocking::{Can as WasiNonblockingCan, Error as WasiNonblockingError};
use wasi::can::types::{
    ErrorCode as WasiCanError, Frame as WasiFrame, FrameKind as WasiFrameKind, Id as WasiId,
};
use wasi::gpio::digital::{
    InputPin as WasiInputPin, OutputPin as WasiOutputPin,
    StatefulOutputPin as WasiStatefulOutputPin,
};
use wasi::gpio::pwm::PwmPin as WasiPwmPin;
use wasi::gpio::types::ErrorCode as WasiGpioError;

// Digital GPIO

impl DigitalError for WasiGpioError {
    fn kind(&self) -> DigitalErrorKind {
        match self {
            WasiGpioError::Other(_) => DigitalErrorKind::Other,
        }
    }
}

impl DigitalErrorType for WasiInputPin {
    type Error = WasiGpioError;
}

impl InputPin for WasiInputPin {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        WasiInputPin::is_high(&self)
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        WasiInputPin::is_low(&self)
    }
}

impl DigitalErrorType for WasiOutputPin {
    type Error = WasiGpioError;
}

impl OutputPin for WasiOutputPin {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        WasiOutputPin::set_low(&self)
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        WasiOutputPin::set_low(&self)
    }
}

impl DigitalErrorType for WasiStatefulOutputPin {
    type Error = WasiGpioError;
}

impl OutputPin for WasiStatefulOutputPin {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        WasiStatefulOutputPin::set_low(&self)
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        WasiStatefulOutputPin::set_high(&self)
    }
}

impl StatefulOutputPin for WasiStatefulOutputPin {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        WasiStatefulOutputPin::is_set_high(&self)
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        WasiStatefulOutputPin::is_set_low(&self)
    }
}

// PWM

impl PwmError for WasiGpioError {
    fn kind(&self) -> PwmErrorKind {
        match self {
            WasiGpioError::Other(_) => PwmErrorKind::Other,
        }
    }
}

impl PwmErrorType for WasiPwmPin {
    type Error = WasiGpioError;
}

impl SetDutyCycle for WasiPwmPin {
    fn max_duty_cycle(&self) -> u16 {
        WasiPwmPin::max_duty_cycle(&self)
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        WasiPwmPin::set_duty_cycle(&self, duty)
    }
}

// CAN frames and identifiers

impl Frame for WasiFrame {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        Some(Self {
            id: id_to_wasi(id.into()),
            kind: WasiFrameKind::Data(data.to_vec()),
        })
    }

    fn new_remote(id: impl Into<Id>, dlc: usize) -> Option<Self> {
        Some(Self {
            id: id_to_wasi(id.into()),
            kind: WasiFrameKind::Remote(dlc as u32),
        })
    }

    fn is_extended(&self) -> bool {
        match self.id {
            WasiId::Standard(_) => false,
            WasiId::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        match &self.kind {
            WasiFrameKind::Data(_) => false,
            WasiFrameKind::Remote(_) => true,
        }
    }

    fn id(&self) -> Id {
        id_from_wasi(self.id)
    }

    fn dlc(&self) -> usize {
        match &self.kind {
            WasiFrameKind::Data(data) => data.len(),
            WasiFrameKind::Remote(dlc) => *dlc as usize,
        }
    }

    fn data(&self) -> &[u8] {
        match &self.kind {
            WasiFrameKind::Data(data) => data,
            WasiFrameKind::Remote(_) => &[],
        }
    }
}

fn id_to_wasi(id: Id) -> WasiId {
    match id {
        Id::Standard(standard_id) => WasiId::Standard(standard_id.as_raw()),
        Id::Extended(extended_id) => WasiId::Extended(extended_id.as_raw()),
    }
}

fn id_from_wasi(id: WasiId) -> Id {
    match id {
        WasiId::Standard(raw) => Id::Standard(
            StandardId::new(raw).expect("wasi:can backend returned an invalid standard identifier"),
        ),
        WasiId::Extended(raw) => Id::Extended(
            ExtendedId::new(raw).expect("wasi:can backend returned an invalid extended identifier"),
        ),
    }
}

// CAN errors

impl CanError for WasiCanError {
    fn kind(&self) -> CanErrorKind {
        match self {
            WasiCanError::Overrun => CanErrorKind::Overrun,
            WasiCanError::Bit => CanErrorKind::Bit,
            WasiCanError::Stuff => CanErrorKind::Stuff,
            WasiCanError::Crc => CanErrorKind::Crc,
            WasiCanError::Form => CanErrorKind::Form,
            WasiCanError::Acknowledge => CanErrorKind::Acknowledge,
            WasiCanError::Other(_) => CanErrorKind::Other,
        }
    }
}

// Blocking CAN

impl BlockingCan for WasiBlockingCan {
    type Frame = WasiFrame;

    type Error = WasiCanError;

    fn transmit(&mut self, frame: &Self::Frame) -> Result<(), Self::Error> {
        WasiBlockingCan::transmit(&self, frame)
    }

    fn receive(&mut self) -> Result<Self::Frame, Self::Error> {
        WasiBlockingCan::receive(&self)
    }
}

// Nonblocking CAN

impl NonblockingCan for WasiNonblockingCan {
    type Frame = WasiFrame;

    type Error = WasiCanError;

    fn transmit(&mut self, frame: &Self::Frame) -> nb::Result<Option<Self::Frame>, Self::Error> {
        match WasiNonblockingCan::transmit(&self, frame) {
            Ok(frame) => Ok(frame),
            Err(err) => match err {
                WasiNonblockingError::WouldBlock => Err(NonblockingError::WouldBlock),
                WasiNonblockingError::Other(error_code) => Err(NonblockingError::Other(error_code)),
            },
        }
    }

    fn receive(&mut self) -> nb::Result<Self::Frame, Self::Error> {
        match WasiNonblockingCan::receive(&self) {
            Ok(frame) => Ok(frame),
            Err(err) => match err {
                WasiNonblockingError::WouldBlock => Err(NonblockingError::WouldBlock),
                WasiNonblockingError::Other(error_code) => Err(NonblockingError::Other(error_code)),
            },
        }
    }
}
