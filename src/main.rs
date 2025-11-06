use anyhow::Result;
use rppal::gpio::Gpio;
use std::thread;
use std::time::Duration;

use crate::robot::UltraSonicSensor;

mod motor;
mod robot;
fn main() -> Result<()> {
    let gpio = Gpio::new()?;
    let mut sensor = UltraSonicSensor::new(&gpio, 17, 18)?;

    println!("Measuring distance...");

    for i in 1..=100 {
        match sensor.measure_distance()? {
            Some(distance) => println!("{}: {:.1} cm", i, distance),
            None => println!("{}: No object detected", i),
        }
        thread::sleep(Duration::from_millis(200));
    }

    Ok(())
}
