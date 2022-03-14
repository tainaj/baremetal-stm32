# Part 7: Embedded C++ Inheritance
This repository is a submit box for Part 7 of Vivonomicon's **"Bare Metal" STM32 Programming** blog series:

https://vivonomicon.com/2018/09/05/bare-metal-stm32-programming-part-7-embedded-c-inheritance/

## Hardware
For this lab, I used the F303K8 Nucleo board; other boards are mentioned in the blog:

Development board | NUCLEO-F303K8
------------------|---------------
Microcontroller   | STM32F303K8T6
Core              | Cortex-M4
Flash memory      | 64 KB
SRAM              | 16 KB<sup>1</sup>
Max frequency     | 72 MHz<sup>2</sup>
Package           | LQFP32

1. 12 KB normal SRAM, 4 KB core coupled memory SRAM (CCMRAM)
2. Due to limits to PLL clock multiplication (see [Part 5](../part5) for clock configuration), this lab will run on 8 MHz instead.

## Software
Get the latest MCU Firmware Package for each supported chip here:
* [STM32F303K8T6](https://github.com/STMicroelectronics/STM32CubeF3 "STM32CubeF3")

Get the latest FreeRTOS Kernel here (this lab uses version 202112.00):
* [FreeRTOS-Kernel](https://github.com/FreeRTOS/FreeRTOS-Kernel "FreeRTOS kernel only")

Consult the **boot_s and vector_tables**, **device_headers**, and **ld** sections in [this guide](../../import-files.md) to import the non-application files for this project. Consult the **freertos** section to import FreeRTOS files.

## Procedure
The following activity is featured in this project:
1. Writing a base IO class.
2. Writing a derived class.
3. Using the classes.
4. Allocating extra memory sections for C++.

Compile the project, and flash to chip with `st-flash write main.bin 0x08000000`. Compare results.

Additional notes within each step is listed below:

### Correction 1: Pure virtual functions
To avoid pulling in more libraries to the project, Vivinomicon passed the following flags to `arm-none-eabi-g++`:
* `-fno-exceptions` - disallow exception handling (try-catch-throw)
* `-fno-rtti` - disable generation of class info during runtime (used by tools like `dynamic-cast`)

These flags still permit the use of pure virtual functions, such as `read()`, `write()`, and `stream()`.
To get them to work in this project:
1. Declare them in the header file (ex. `virtual unsigned read(void) = 0;`)
2. Do not define them in the source file (ex. `read()` commented out)
3. Add `__cxa_pure_virtual() { while(1); }` in `main.cpp`

### Correction 2: Task priority values
In `main.cpp`, in `xTaskCreate()`.
* Vivonomicon sets tasks to priorities `configMAX_PRIORITIES-7`, `configMAX_PRIORITIES-6`, and `configMAX_PRIORITIES-5`.
* `FreeRTOSConfig.h` defines `configMAX_PRIORITIES` to 5, so the above results in priorities of -2, -1, and 0.
* `xTaskCreate()` accepts a priority from 0 to ( configMAX_PRIORITIES - 1 ). In this case, from 0 to 4.

The original code does not run. This issue was resolved by adding 4 to all, resulting in priorities 2, 3, and 4.

## Author's notes
Straightforward. Note both corrections when troubleshooting.
