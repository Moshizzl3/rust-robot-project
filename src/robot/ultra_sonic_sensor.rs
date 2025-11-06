use std::{
    thread,
    time::{Duration, Instant},
};

use anyhow::{Context, Ok, Result};
use rppal::gpio::{Gpio, InputPin, OutputPin};

const SPEED_OF_SOUND: f64 = 0.0343; //cm pr microsec

pub struct UltraSonicSensor {
    trigger: OutputPin,
    echo: InputPin,
}

impl UltraSonicSensor {
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

    /// Measures distance in centimeters
    /// Returns None if no object detected (timeout)
    pub fn measure_distance(&mut self) -> Result<Option<f64>> {
        // ensure trigger is low
        self.trigger.set_low();
        thread::sleep(Duration::from_micros(5));

        // Send 10µs trigger pulse
        self.trigger.set_high();
        thread::sleep(Duration::from_micros(10));
        self.trigger.set_low();

        // Wait for echo to go high
        let wait_start = Instant::now();
        while self.echo.is_low() {
            if wait_start.elapsed().as_micros() > 1000 {
                return Ok(None); // No echo received 
            }
        }

        // wait for it to go back LOW, the duration is for the distance calc
        let pulse_start = Instant::now();
        while self.echo.is_high() {
            if pulse_start.elapsed().as_micros() > 30000 {
                return Ok(None); // Error or target too far, avoid looping forever
            }
        }
        let elapsed_time = pulse_start.elapsed().as_micros();

        //divide by two since we need to take into account the time travled to and from the object
        let distance = (elapsed_time as f64 * SPEED_OF_SOUND) / 2.0;

        Ok(Some(distance))
    }
}
