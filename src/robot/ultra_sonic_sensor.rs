use anyhow::{Context, Result};
use rppal::gpio::{Gpio, InputPin, OutputPin};

const SPEED_OF_SOUND: f64 = 0.0343; //cm pr microsec

pub struct UltraSonicCensor {
    trigger: OutputPin,
    echo: InputPin,
}

impl UltraSonicCensor {
    pub fn new(gpio: &Gpio, trigger_pin: u8, echo_pin: u8) -> Result<Self> {
        Ok(Self {
            trigger: gpio
                .get(trigger_pin)
                .context(format!("Failed to create trigger pin: {}", trigger_pin))?
                .into_output(),
            echo: gpio
                .get(echo_pin)
                .context(format!("Failed to create echo pin: {}", echo_pin))?
                .into_input(),
        })
    }
}
