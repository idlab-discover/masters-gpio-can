use embedded_hal::pwm::{ErrorKind, ErrorType, SetDutyCycle};
use sysfs_pwm::Pwm;

const MAX_DUTY: u16 = u16::MAX;

#[derive(Debug)]
pub struct SysfsPwm {
    pwm: Pwm,
}

impl SysfsPwm {
    pub fn new(chip: u32, channel: u32, period_ns: u32) -> sysfs_pwm::Result<Self> {
        let pwm = Pwm::new(chip, channel)?;
        pwm.export()?;
        pwm.enable(false)?;
        pwm.set_duty_cycle_ns(0)?;
        pwm.set_period_ns(period_ns)?;
        pwm.enable(true)?;

        Ok(Self { pwm })
    }
}

impl ErrorType for SysfsPwm {
    type Error = ErrorKind;
}

impl SetDutyCycle for SysfsPwm {
    fn max_duty_cycle(&self) -> u16 {
        MAX_DUTY
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        let period_ns = self.pwm.get_period_ns().map_err(|_| ErrorKind::Other)?;
        let duty_cycle_ns = u64::from(duty) * u64::from(period_ns) / u64::from(MAX_DUTY);

        self.pwm
            .set_duty_cycle_ns(duty_cycle_ns as u32)
            .map_err(|_| ErrorKind::Other)
    }
}
