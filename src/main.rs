use rppal::gpio::Gpio;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing GPIO...");
    let gpio = Gpio::new()?;

    // left motor
    let mut left_forward = gpio.get(7)?.into_output();
    let mut left_backward = gpio.get(8)?.into_output();

    // right motor
    let mut right_forward = gpio.get(9)?.into_output();
    let mut right_backward = gpio.get(10)?.into_output();

    println!("Left motor forward for 2 seconds...");
    left_forward.set_high();
    left_backward.set_low();
    right_forward.set_low();
    right_backward.set_low();

    thread::sleep(Duration::from_secs(2));

    println!("Stopping...");
    left_forward.set_low();

    thread::sleep(Duration::from_secs(1));

    println!("Right motor forward for 2 seconds...");
    right_forward.set_high();

    thread::sleep(Duration::from_secs(2));

    println!("Stopping...");
    right_forward.set_low();

    println!("Done!");
    println!("Both motors forward for 2 seconds...");

    // Set left motor
    left_forward.set_high();
    left_backward.set_low();

    // set right motor
    right_forward.set_high();
    right_backward.set_low();

    // wait (both running)
    thread::sleep(Duration::from_secs(2));

    println!("Stopping both...");
    left_forward.set_low();
    right_forward.set_low();

    Ok(())
}
