use anyhow::Result;
use rppal::gpio::Gpio;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    let gpio = Gpio::new()?;

    let mut trigger = gpio.get(17)?.into_output();
    let echo = gpio.get(18)?.into_input();

    println!("sending 100 trigger pulses...\n");

    for i in 1..=100 {
        println!("Pulse {}", i);

        // ensure trigger is low
        trigger.set_low();
        thread::sleep(Duration::from_micros(5));

        // Send 10µs trigger pulse
        trigger.set_high();
        thread::sleep(Duration::from_micros(10));
        trigger.set_low();

        // check if echo responds
        println!("Waiting for echo to go HIGH...");

        let mut timeout = 0;
        while echo.is_low() && timeout < 1000 {
            thread::sleep(Duration::from_micros(1));
            timeout += 1;
        }

        if timeout >= 1000 {
            println!("Echo never went HIGH (timeout)");
        } else {
            println!("Echo went HIGH!");

            // eait for it to go back LOW
            let mut duration = 0;
            while echo.is_high() && duration < 10000 {
                thread::sleep(Duration::from_micros(1));
                duration += 1;
            }

            println!("Echo pulse duration: ~{} µs", duration);
        }

        println!();
        thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}
