use rppal::gpio::{Gpio, Level};
use std::error::Error;
use std::thread;
use std::time::Duration;
mod motor;
mod robot;
use motor::Motor;
use robot::Robot;
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
    println!("Line sensor test - GPIO 25");
    let gpio = Gpio::new()?;

    // pin 25 as input from docs
    let line_sensor = gpio.get(25)?.into_input_pullup();

    println!("Move sensor over black and white surfaces...");
    println!("Press Ctrl+C to exit\n");

    loop {
        let level = line_sensor.read();

        match level {
            Level::High => println!("is high"),
            Level::Low => println!("is low"),
        }
        thread::sleep(Duration::from_millis(500));
    }
}
