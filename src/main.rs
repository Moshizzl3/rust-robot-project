use rppal::gpio::{Gpio, Level};
use std::error::Error;
use std::thread;
use std::time::Duration;
mod motor;
mod robot;
use motor::Motor;
use robot::{LineSensor, Robot};
// fn main() -> Result<(), Box<dyn Error>> {
//     let gpio = Gpio::new()?;
//     let mut robot = Robot::new(&gpio, "Rusty".to_string())?;

//     robot.forward();
//     thread::sleep(Duration::from_secs(2));

//     robot.spin_left();
//     thread::sleep(Duration::from_secs(1));

//     robot.spin_right();
//     thread::sleep(Duration::from_secs(1));

//     robot.backward();
//     thread::sleep(Duration::from_secs(2));

//     robot.turn_right();
//     thread::sleep(Duration::from_secs(2));

//     robot.turn_left();
//     thread::sleep(Duration::from_secs(2));

//     robot.stop();

//     Ok(())
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gpio = Gpio::new()?;
    let sensor = LineSensor::new(&gpio)?;

    println!("Testing LineSensor struct...");
    println!("Press Ctrl+C to exit\n");

    loop {
        if sensor.is_on_line() {
            println!("On BLACK line");
        } else {
            println!("On WHITE surface");
        }

        thread::sleep(Duration::from_millis(500));
    }
}
