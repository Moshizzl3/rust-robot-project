use std::error::Error;

use rppal::gpio::{Gpio, InputPin};

pub struct LineSensor {
    pin: InputPin,
}

impl LineSensor {
    pub fn new(gpio: &Gpio, pin_number: u8) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            pin: gpio.get(pin_number)?.into_input(),
        })
    }

    pub fn is_on_line(&self) -> bool {
        self.pin.is_low()
    }

    pub fn is_on_white(&self) -> bool {
        self.pin.is_high()
    }
}
