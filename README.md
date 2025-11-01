# Autonomous Obstacle-Avoiding Robot

An embedded Rust project building a fully autonomous robot using a Raspberry Pi 4 and CamJam EduKit 3. This project focuses on learning embedded systems programming through hands-on hardware control, starting from basic motor control and progressively adding obstacle avoidance and advanced navigation features.

## Hardware

- **Raspberry Pi 4 (8GB)** - Main controller running Raspberry Pi OS
- **CamJam EduKit 3 Robotics Kit**
  - 2x DC Motors with wheels
  - Motor controller board (H-bridge)
  - Ultrasonic distance sensor (HC-SR04)
  - Line following sensors
  - Battery pack (4x AA batteries)
  - Chassis and hardware

## Project Goals

- **Learn embedded Rust** through practical, hands-on development
- **Understand hardware fundamentals**: GPIO, PWM, sensors, and real-time constraints
- **Build progressively**: Start simple, add complexity incrementally
- **Create portfolio-worthy project** with full documentation
- **Document the learning journey** for others and future reference

## Development Background

**My Experience Coming In:**
- Comfortable with Linux/networking (built Kubernetes cluster on Raspberry Pis)
- Beginner-intermediate Rust (can code but still developing fluency)
- No prior embedded/hardware experience
- Dedicated 30+ hours/week to this project

## Development Log

### Week 1: Days 1-2 - Foundation & First Movement

#### Day 1: Environment Setup & Core Concepts

**Topics Researched:**
- **GPIO (General Purpose Input/Output)**: Understanding how software controls physical pins
- **BCM2711 SoC Architecture**: The Broadcom chip that powers the Pi 4
- **PWM (Pulse Width Modulation)**: How to control motor speed with digital signals
- **Memory-Mapped I/O**: How GPIO writes bypass the kernel for direct hardware access
- **H-Bridge Motor Drivers**: How 4 transistors enable bidirectional motor control

**Key Conceptual Learnings:**
- GPIO pins on the Pi output 3.3V when HIGH, 0V when LOW
- Each GPIO pin can only safely source ~16mA (motors need 100-500mA+)
- This is why motor controller boards are essential - they amplify the signal
- Two numbering schemes exist: Physical pin numbers (1-40) vs BCM GPIO numbers
- **Critical**: Software always uses BCM GPIO numbers, not physical positions

**Setup Completed:**
- Installed Rust on Raspberry Pi using rustup
- Created initial `robot_controller` project with Cargo
- Added `rppal` crate (Raspberry Pi Peripheral Access Library)
- Configured VS Code with Remote-SSH for development

#### Day 2: First Motor Control & Hardware Validation

**Major Milestone:** Got motors spinning! 🎉

**Process:**
1. Found CamJam EduKit 3 documentation and extracted pin assignments
2. Discovered documentation had forward/backward pins reversed
3. Empirically tested and corrected pin mappings through iteration
4. Successfully controlled individual motors
5. Made both motors run simultaneously

**Hardware Reality Check:**
Documentation said one thing, hardware did another. This taught an important lesson: **always verify with real hardware testing**. Wrote simple test code to validate each motor direction independently before trusting the docs.

**What Works Now:**
- Individual motor control (forward/backward/stop)
- Simultaneous dual motor operation
- Basic straight-line driving

**Challenges Faced:**
- Initial confusion between physical pin numbers and BCM GPIO numbering
- CamJam documentation had reversed pin labels for our motor configuration
- Understanding why both motors started when we expected only one (needed to explicitly set unused motor pins LOW)

**Key Code Insight:**
Motor control is instantaneous - `set_high()` and `set_low()` execute in microseconds. The `thread::sleep()` doesn't control the motors; it just pauses the program while motors continue running based on their last pin state.

## Pin Configuration

**Pin Numbering:** Using BCM GPIO numbers (not physical pin positions)

**Left Motor:**
- Forward: GPIO 7 (Physical Pin 26)
- Backward: GPIO 8 (Physical Pin 24)

**Right Motor:**
- Forward: GPIO 9 (Physical Pin 21)
- Backward: GPIO 10 (Physical Pin 19)

**Power:**
- Motor controller powered by 4x AA battery pack (separate from Pi)
- Pi powered via USB-C

## Current Implementation

**Language & Framework:**
- Rust (stable)
- `rppal` crate v0.19 for GPIO access

**Architecture:**
- Running on Raspberry Pi OS (full Linux, not bare-metal)
- Using standard Rust (`std`) with Linux hardware access
- GPIO control via memory-mapped `/dev/gpiomem`

## Running the Code
```bash
# Build the project
cargo build

# Run (requires root for GPIO access)
sudo ./target/debug/robot_controller
```

**Note:** `sudo` is required because GPIO access needs root permissions to open `/dev/gpiomem`.

## What I've Learned So Far

### Technical Concepts

1. **Memory-Mapped I/O is Fast**
   - Direct hardware access without kernel context switches
   - GPIO registers mapped to memory addresses (e.g., 0xFE200000)
   - Single CPU instruction from code to hardware

2. **H-Bridges Enable Motor Reversal**
   - 4 transistors arranged in an H configuration
   - Control which direction current flows through motor
   - "Shoot-through" (both sides on) causes short circuit - must avoid

3. **PWM Provides Speed Control**
   - Can't just lower voltage (motors need minimum voltage to overcome friction)
   - PWM delivers full voltage in pulses
   - Motor inertia averages the power over time
   - Higher frequency = smoother operation

4. **Rust's Ownership Prevents Hardware Conflicts**
   - Can't get the same GPIO pin twice
   - Compile-time safety for hardware resources
   - `rppal` tracks pin usage and returns errors if pin already in use

### Practical Lessons

- **Documentation isn't always accurate** - test your hardware empirically
- **Start with the simplest possible test** - one motor, full speed, no PWM
- **Physical pin numbers ≠ GPIO numbers** - always clarify which you're using
- **Hardware debugging is different** - can't just add print statements, need to verify electrical state

## Next Steps

### Immediate (Day 3):
- [ ] Refactor working code into a `Motor` struct
- [ ] Implement clean API: `forward()`, `backward()`, `stop()`
- [ ] Create `Robot` struct to manage both motors
- [ ] Add basic turning capabilities (left/right)

### Short Term (Week 1-2):
- [ ] Implement software PWM for variable speed control
- [ ] Add ultrasonic sensor for distance measurement
- [ ] Basic obstacle detection and stopping

### Medium Term (Week 2-4):
- [ ] Autonomous obstacle avoidance algorithms
- [ ] Line following using line sensors
- [ ] Speed calibration (motors aren't perfectly matched)

### Long Term:
- [ ] Advanced navigation (mapping, path planning)
- [ ] Remote control interface
- [ ] Data logging and visualization

## Resources Used

**Official Documentation:**
- [CamJam EduKit 3 Documentation](https://github.com/CamJam-EduKit/EduKit3)
- [Raspberry Pi GPIO Documentation](https://www.raspberrypi.com/documentation/computers/raspberry-pi.html)
- [rppal Crate Documentation](https://docs.rs/rppal/)

**Learning Resources:**
- BCM2711 ARM Peripherals datasheet for low-level GPIO details
- H-bridge motor driver theory and operation
- PWM fundamentals for motor speed control
- Rust ownership and borrowing patterns for hardware access

## Why Rust for Embedded?

Choosing Rust for this project because:
- **Memory safety without garbage collection** - critical for real-time systems
- **Ownership model prevents hardware conflicts** - can't accidentally use same pin twice
- **Zero-cost abstractions** - clean code that compiles to efficient machine code
- **Great embedded ecosystem** - `rppal`, `embedded-hal`, and more
- **Learning opportunity** - combines systems programming with hardware control
- **I like Rust** - just ❤️🦀


## Acknowledgments

- CamJam EduKit team for the excellent robotics kit and documentation
- The `rppal` crate maintainers for comprehensive Raspberry Pi peripheral access
- Rust embedded working group for pushing embedded Rust forward