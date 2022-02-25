# Original 2 - TC2004A-I2C LCD text display
This repository is a C++ to C conversion of a LiquidCrystal Arduino library for I2C LCD displays, sourced from John Rickman. Link to Rickman's repository here:

https://github.com/johnrickman/LiquidCrystal_I2C

## Hardware
For this lab, I used both the NUCLEO-L031K6 and the STM32F0DISCOVERY used in previous labs. Users are free to adapt the code for other STM32 devices.

## Software
Links to STMicroelectronics' MCU Firmware Package for each supported chip are below:
* [STM32F051R8T6](https://github.com/STMicroelectronics/STM32CubeF0 "STM32CubeF0")
* [STM32L031K6T6](https://github.com/STMicroelectronics/STM32CubeL0 "STM32CubeL0")

## Operation
When the STM32 is booted, the STM32 will perform the following:
* Initial clock setup - F0 devices run at 48 MHz, L0 devices run at 32 MHz.
* Configure GPIO pins to I2C peripheral (define pin mappings in `i2c.h`)
* Run one of three available tests (Set `TEST` number 1, 2, or 3)
  1. Hello World - populate all four lines with text
  2. Custom char - print custom characters (see [this post](https://www.handsonembedded.com/lcd16x2-hd44780-tutorial-5/) to learn how to compose your own characters!)
  3. Experiment - demonstrate various module commands

Compile the project with Make, and flash the project with either `st-flash` or STM32CubeProgrammer.

## Author's notes 
Documentation for the TC2004-A datasheet [here.](https://cdn-shop.adafruit.com/datasheets/TC2004A-01.pdf "TC2004-A Datasheet")

Commands and data use data pins D7 to D0, and 4 other pins (BL = backlight, E = enable, RW = read/write, RS = register select) are used. But the module is interfaced with an 8-bit I/O expander for I2C-bus.

So the TC2004 module is configured to 4-bit; when a command/data byte is passed to send() to be transfered via I2C, two transfers are required:

Transfer #   | Bit 7 (MSB) | Bit 6 | Bit 5 | Bit 4 | Bit 3 | Bit 2 | Bit 1 | Bit 0 (LSB)
-------------|-------------|-------|-------|-------|-------|-------|-------|-------------
1st transfer | D7          | D6    | D5    | D4    | BL    | E     | RW    | RS
2nd transfer | D3          | D2    | D1    | D0    | BL    | E     | RW    | RS

To ensure no glitches happen when shifting command/data to D7:D0, the module actually uses 6 transfers:
* (1) upper bits E=0
* (2) upper bits E=1
* (3) upper bits E=0
* (4-6) repeat for lower bits

This ensures that the correct command/data is present in the I/O expander when TC2004A reads it (E=1). See Write Mode Timing Diagram (p. 7)
