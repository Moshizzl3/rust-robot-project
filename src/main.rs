use rppal::gpio::Gpio;
use std::thread;
use std::time::Duration;
mod motor;
use crate::motor::motor::Motor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing GPIO...");
    let gpio = Gpio::new()?;

    // left motor
    let mut left_motor = Motor::new(&gpio, 7, 8)?;
    // right motor
    let mut right_motor = Motor::new(&gpio, 9, 10)?;

    println!("Both motors forward for 2 seconds...");

    // Set left motor
    left_motor.forward();

    // set right motor
    right_motor.forward();

    // wait (both running)
    thread::sleep(Duration::from_secs(2));

    println!("Stopping both...");
    left_motor.stop();
    right_motor.stop();

    // Set left motor
    left_motor.backward();
    // set right motor
    right_motor.backward();
    // wait (both running)
    thread::sleep(Duration::from_secs(2));
    left_motor.stop();
    right_motor.stop();

    Ok(())
}
