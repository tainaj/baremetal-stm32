# Original 1.5 - Delay module 2.0
This repository contains a timer-based implementation of `delay()`, a function that stalls the processor for a configurable duration. See here for an earlier implementation...

## Hardware
For this lab, I used both the NUCLEO-L031K6 and the STM32F0DISCOVERY used in previous labs. Users are free to adapt the code for other STM32 devices.

## Software
Links to STMicroelectronics' MCU Firmware Package for each supported chip are below:
* [STM32F051R8T6](https://github.com/STMicroelectronics/STM32CubeF0 "STM32CubeF0")
* [STM32L031K6T6](https://github.com/STMicroelectronics/STM32CubeL0 "STM32CubeL0")

To find `boot_code`, `device_headers`, and `vector_table` files, follow the guide under the **Software** header in [Part 3's README](../part3).

## Operation
When the STM32 is booted, the LED pin will toggle after every second.

Compile the project with Make, and flash the project with wither `st-flash` or STM32CubeProgrammer.

## Author's notes 
This implementation of `delay()` seperates millisecond and microsecond durations with `milli_wait()` and `micro_wait()`, respectively. Some notes about these functions:
* SysTick and TIM2 (can be a different clock) must be initialized. The clock speed global `core_clock_hz` must be passed as a parameter.
* `milli_wait()` uses a 32-bit input parameter in milliseconds; start and end times are derived from reading `_millis`, incremented every 1 ms by SysTick interrupt.
* `micro_wait()` uses a 16-bit input parameter in microseconds; start and end times are derived from reading `TIM2->CNT`, TIM2's counter value incremented every 1 us by prescaler.
