use rppal::gpio::{Gpio, Level};
use std::error::Error;
use std::thread;
use std::time::Duration;
mod motor;
mod robot;
use motor::Motor;
use robot::{LineSensor, Robot};

fn main() -> Result<(), Box<dyn Error>> {
    let gpio = Gpio::new()?;
    let mut robot = Robot::new(&gpio, "Rusty".to_string())?;

    println!("Starting line follower...m");
    println!("Press Ctrl+C to stop");

    loop {
        robot.follow_line_pulse(20);
        thread::sleep(Duration::from_millis(50)); // How fast to check?
    }
    robot.stop();

    Ok(())
}
