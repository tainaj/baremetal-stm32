# Original 1 - Delay module
This repository contains an early implementation of `delay()`, a function that stalls the processor for a configurable duration. See here for a better implementation...

## Hardware
For this lab, I used both the NUCLEO-L031K6 and the STM32F0DISCOVERY  used in previous labs. Users are free to adapt the code for other STM32 devices.

## Software
Links to STMicroelectronics' MCU Firmware Package for each supported chip are below:
* [STM32F051R8T6](https://github.com/STMicroelectronics/STM32CubeF0 "STM32CubeF0")
* [STM32L031K6T6](https://github.com/STMicroelectronics/STM32CubeL0 "STM32CubeL0")

To find `boot_code`, `device_headers`, and `vector_table` files, follow the guide under the **Software** header in [Part 3's README](../part3).

## Operation
When the STM32 is booted, the LED pin will toggle after every second.

Compile the project with Make, and flash the project with wither `st-flash` or STM32CubeProgrammer.

## Author's notes 
This implementation of `delay()` is called from assembly file `delay_$(MCU_CLASS).S` (see Makefile). Some notes about these files:
* The input parameter for `delay` is an int-type, specified in microseconds.
* `delay_F0.S` and `delay_L0.S` assumes the clock speed is 48 MHz and 32 MHz, respectively.
* This implementation is to be used as a close approximation only; see the other implementation linked above.
* Otherwise, adjust the '1-second' duration by tweaking the number of nops in each .S file.
