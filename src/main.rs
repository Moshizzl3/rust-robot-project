use anyhow::{Ok, Result};
use rppal::gpio::Gpio;
use std::thread;
use std::time::Duration;

use crate::robot::{Robot, UltraSonicSensor};

mod motor;
mod robot;
fn main() -> Result<()> {
    let gpio = Gpio::new()?;
    let mut sensor = UltraSonicSensor::new(&gpio, 17, 18)?;
    let mut robot = Robot::new(&gpio, "rusty".to_string())?;

    loop {
        let distance = sensor.measure_distance()?.unwrap_or(0.0);

        if distance > 30.0 {
            robot.forward(0.5)?
        } else {
            robot.stop()?;
            thread::sleep(Duration::from_millis(500));
            robot.backward(0.5)?;
            thread::sleep(Duration::from_millis(500));
            robot.turn_right()?;
            thread::sleep(Duration::from_millis(500));
            if distance > 30.0 {
                break;
            }
        }

        thread::sleep(Duration::from_millis(500));
    }
    robot.stop()?;
    Ok(())
}
