use rppal::gpio::{Gpio, Level};
use std::error::Error;
use std::thread;
use std::time::Duration;
mod motor;
mod robot;
use motor::Motor;
use robot::{LineSensor, Robot};

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_PWM: u8 = 7;

// Servo configuration. Change these values based on your servo's verified safe
// minimum and maximum values.
//
// Period: 20 ms (50 Hz). Pulse width: min. 1200 µs, neutral 1500 µs, max. 1800 µs.
const PERIOD_MS: u64 = 20;
const PULSE_MIN_US: u64 = 1200;
const PULSE_NEUTRAL_US: u64 = 1500;
const PULSE_MAX_US: u64 = 1800;

fn main() -> Result<(), Box<dyn Error>> {
    let mut pin = Gpio::new()?.get(GPIO_PWM)?.into_output();

    pin.set_pwm_frequency(1000.0, 0.50)?;

    thread::sleep(Duration::from_millis(1500));

    Ok(())
}
