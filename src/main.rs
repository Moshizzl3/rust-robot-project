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

    let trigger_pin = gpio.get(17)?.into_input();
    let echo_pin = gpio.get(18)?.into_output();

    loop {
        println!("wooow: {}", trigger_pin.is_high());
        println!("wooow: {}", echo_pin.)
    }

    Ok(())
}
