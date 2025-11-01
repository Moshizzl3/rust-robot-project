use std::error::Error;

use rppal::gpio::{Gpio, InputPin};

const LINE_SENSOR_PIN: u8 = 25;

pub struct LineSensor {
    pin: InputPin,
}

impl LineSensor {
    pub fn new(gpio: &Gpio) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            pin: gpio.get(LINE_SENSOR_PIN)?.into_input(),
        })
    }

    pub fn is_on_line(&self) -> bool {
        self.pin.is_low()
    }

    pub fn is_on_white(&self) -> bool {
        self.pin.is_high()
    }
}
