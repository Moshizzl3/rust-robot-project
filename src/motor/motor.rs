use anyhow::{Context, Result};
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
    pub fn new(gpio: &Gpio, backward_pin: u8, forward_pin: u8) -> Result<Self> {
        Ok(Self {
            gpio_backward_pin: gpio
                .get(backward_pin)
                .context(format!("Failed to create backward pin: {}", backward_pin))?
                .into_output(),

            gpio_forward_pin: gpio
                .get(forward_pin)
                .context(format!("Failed to create forward pin: {}", forward_pin))?
                .into_output(),
        })
    }
    ///  Make motor go forward
    /// # Arguments
    /// * `speed` - speed controller from 0.0-1.0
    pub fn forward(&mut self, speed: f64) -> Result<()> {
        let speed = speed.clamp(0.0, 1.0);

        self.gpio_backward_pin
            .clear_pwm()
            .context("Failed to stop backward pin")?;

        self.gpio_forward_pin
            .set_pwm_frequency(FREQUENCY, speed)
            .context(format!("Failed to set forward speed: {}", speed))?;

        Ok(())
    }

    /// Make motor go backward
    /// # Arguments
    /// * `speed` - speed controller from 0.0-1.0
    pub fn backward(&mut self, speed: f64) -> Result<()> {
        let speed = speed.clamp(0.0, 1.0);

        self.gpio_forward_pin
            .clear_pwm()
            .context("Failed to stop forward pin")?;

        self.gpio_backward_pin
            .set_pwm_frequency(FREQUENCY, speed)
            .context(format!("Failed to set backward speed: {}", speed))?;

        Ok(())
    }

    /// Stop the motor
    pub fn stop(&mut self) -> Result<()> {
        self.gpio_forward_pin
            .clear_pwm()
            .context("Failed to stop forward pin")?;

        self.gpio_backward_pin
            .clear_pwm()
            .context("Failed to stop backward pin")?;

        Ok(())
    }
}
