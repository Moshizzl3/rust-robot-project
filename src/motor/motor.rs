use std::error::Error;

use rppal::gpio::{Gpio, OutputPin};

/// Represents a DC motor controlled by two GPIO pins
pub struct Motor {
    gpio_backward_pin: OutputPin,
    gpio_forward_pin: OutputPin,
}

impl Motor {
    /// Creates a new motor
    ///
    /// # Arguments
    /// * `gpio` - GPIO controller
    /// * `backward_pin` - GPIO pin for backward direction
    /// * `forward_pin` - GPIO pin for forward direction
    pub fn new(gpio: &Gpio, backward_pin: u8, forward_pin: u8) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            gpio_backward_pin: gpio.get(backward_pin)?.into_output(),
            gpio_forward_pin: gpio.get(forward_pin)?.into_output(),
        })
    }
    ///  Make motor go forward
    pub fn forward(&mut self) {
        self.gpio_forward_pin.set_high();
        self.gpio_backward_pin.set_low();
    }

    /// Make motor go backward
    pub fn backward(&mut self) {
        self.gpio_forward_pin.set_low();
        self.gpio_backward_pin.set_high();
    }

    /// Stop the motor
    pub fn stop(&mut self) {
        self.gpio_forward_pin.set_low();
        self.gpio_backward_pin.set_low();
    }
}
