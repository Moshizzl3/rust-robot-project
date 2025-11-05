use std::error::Error;

use rppal::gpio::{Gpio, OutputPin};

const FREQUENCY: f64 = 2000.0;

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
    pub fn forward(&mut self, speed: f64) {
        self.gpio_forward_pin.set_pwm_frequency(FREQUENCY, speed);
    }

    /// Make motor go backward
    pub fn backward(&mut self, speed: f64) -> Result<(), Box<dyn Error>> {
        self.gpio_backward_pin.set_pwm_frequency(FREQUENCY, speed)?;
        Ok(())
    }

    /// Stop the motor
    pub fn stop(&mut self) {
        self.gpio_forward_pin.clear_pwm();
        self.gpio_backward_pin.clear_pwm();
    }
}
