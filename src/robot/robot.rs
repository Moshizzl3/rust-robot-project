use std::error::Error;

use rppal::gpio::Gpio;

use crate::motor::Motor;

const LEFT_MOTOR_PINS: (u8, u8) = (7, 8);
const RIGHT_MOTOR_PINS: (u8, u8) = (9, 10);

pub struct Robot {
    pub name: String,
    left_motor: Motor,
    right_motor: Motor,
}

impl Robot {
    pub fn new(gpio: &Gpio, name: String) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            name,
            left_motor: Motor::new(gpio, LEFT_MOTOR_PINS.0, LEFT_MOTOR_PINS.1)?,
            right_motor: Motor::new(gpio, RIGHT_MOTOR_PINS.0, RIGHT_MOTOR_PINS.1)?,
        })
    }

    pub fn forward(&mut self) {
        println!("{} driving forward..", self.name);
        self.left_motor.forward();
        self.right_motor.forward();
    }
    pub fn backward(&mut self) {
        println!("{} driving backwards..", self.name);
        self.left_motor.backward();
        self.right_motor.backward();
    }

    pub fn turn_right(&mut self) {
        println!("{} turning right..", self.name);
        self.left_motor.forward();
        self.right_motor.stop();
    }

    pub fn turn_left(&mut self) {
        println!("{} turning left..", self.name);
        self.right_motor.forward();
        self.left_motor.stop();
    }

    pub fn spin_left(&mut self) {
        println!("{} spinning left..", self.name);
        self.right_motor.forward();
        self.left_motor.backward();
    }

    pub fn spin_right(&mut self) {
        println!("{} spinning right..", self.name);
        self.left_motor.forward();
        self.right_motor.backward();
    }

    pub fn stop(&mut self) {
        println!("{} stoping now..", self.name);
        self.left_motor.stop();
        self.right_motor.stop();
    }
}
