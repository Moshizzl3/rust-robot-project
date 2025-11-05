use std::{error::Error, thread, time::Duration};

use rppal::gpio::Gpio;

use crate::{motor::Motor, robot::LineSensor};

const LEFT_MOTOR_PINS: (u8, u8) = (7, 8);
const RIGHT_MOTOR_PINS: (u8, u8) = (9, 10);
const LINE_SENSOR_PIN: u8 = 25;

pub struct Robot {
    pub name: String,
    left_motor: Motor,
    right_motor: Motor,
    line_sensor: LineSensor,
}

impl Robot {
    pub fn new(gpio: &Gpio, name: String) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            name,
            left_motor: Motor::new(gpio, LEFT_MOTOR_PINS.0, LEFT_MOTOR_PINS.1)?,
            right_motor: Motor::new(gpio, RIGHT_MOTOR_PINS.0, RIGHT_MOTOR_PINS.1)?,
            line_sensor: LineSensor::new(gpio, LINE_SENSOR_PIN)?,
        })
    }

    pub fn forward(&mut self, speed: f64) {
        println!("{} driving forward..", self.name);
        self.left_motor.forward(speed);
        self.right_motor.forward(speed);
    }
    pub fn backward(&mut self, speed: f64) {
        println!("{} driving backwards..", self.name);
        self.left_motor.backward(speed);
        self.right_motor.backward(speed);
    }

    pub fn turn_right(&mut self) {
        println!("{} turning right..", self.name);
        self.left_motor.forward(0.5);
        self.right_motor.forward(0.2);
    }

    pub fn turn_left(&mut self) {
        println!("{} turning left..", self.name);
        self.right_motor.forward(0.5);
        self.left_motor.forward(0.2);
    }

    pub fn spin_left(&mut self) {
        println!("{} spinning left..", self.name);
        self.right_motor.forward(0.5);
        self.left_motor.backward(0.5);
    }

    pub fn spin_right(&mut self) {
        println!("{} spinning right..", self.name);
        self.left_motor.forward(0.5);
        self.right_motor.backward(0.5);
    }

    pub fn stop(&mut self) {
        println!("{} stoping now..", self.name);
        self.left_motor.stop();
        self.right_motor.stop();
    }

    pub fn follow_line(&mut self, speed: f64) {
        if self.line_sensor.is_on_line() {
            self.forward(speed);
        } else {
            self.turn_right();
        }
    }
}
