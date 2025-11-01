use rppal::gpio::Gpio;
use std::error::Error;
use std::thread;
use std::time::Duration;
mod motor;
mod robot;
use motor::Motor;
use robot::Robot;
fn main() -> Result<(), Box<dyn Error>> {
    let gpio = Gpio::new()?;
    let mut robot = Robot::new(&gpio, "Rusty".to_string())?;

    robot.forward();
    thread::sleep(Duration::from_secs(2));

    robot.spin_left();
    thread::sleep(Duration::from_secs(1));

    robot.spin_right();
    thread::sleep(Duration::from_secs(1));

    robot.backward();
    thread::sleep(Duration::from_secs(2));

    robot.turn_right();
    thread::sleep(Duration::from_secs(2));

    robot.turn_left();
    thread::sleep(Duration::from_secs(2));

    robot.stop();

    Ok(())
}
