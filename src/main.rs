use rppal::gpio::{Gpio, Level};
use std::error::Error;
use std::thread;
use std::time::Duration;
mod motor;
mod robot;
use motor::Motor;
use robot::{LineSensor, Robot};

use anyhow::Result;

fn main() -> Result<()> {
    let gpio = Gpio::new()?;
    let mut robot = Robot::new(&gpio, "rusty".to_string())?;

    println!("Forward at 50%");
    robot.forward(0.5)?;
    thread::sleep(Duration::from_secs(2));

    println!("forward at 100%");
    robot.forward(1.0)?;
    thread::sleep(Duration::from_secs(2));

    println!("Backward at 30%");
    robot.backward(0.5)?;
    thread::sleep(Duration::from_secs(2));

    robot.stop()?;
    Ok(())
}
