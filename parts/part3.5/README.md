# Part 3.5: Supporting Multiple Chips
This repository is a submit box for Part 3.5 of Vivonomicon's **"Bare Metal" STM32 Programming** blog series:

https://vivonomicon.com/2018/04/25/bare-metal-stm32-programming-part-3-5-supporting-multiple-chips/

## Hardware
For this lab, I used the STM32F0 Discovery board and Nucleo L031K6 board; other boards are mentioned in the blog:

Development board | STM32F0DISCOVERY | NUCLEO-F031K6 | NUCLEO-L031K6
------------------|------------------|---------------|---------------
Microcontroller   | STM32F051R8T6    | STM32F031K6   | STM32L031K6T6
Core              | Cortex-M0        | Cortex-M0     | Cortex-M0+
Flash memory      | 64 KB            | 32 KB         | 32 KB
SRAM              | 8 KB             | 4 KB          | 8 KB
Max frequency     | 48 MHz           | 48 MHz        | 32 MHz
Package           | LQFP64           | LQFP32        | LQFP32

For future projects, I will omit development boards from the table that I have not used in the project.

## Software
Links to STMicroelectronics' MCU Firmware Package for each supported chip are below:
* [STM32F051R8T6](https://github.com/STMicroelectronics/STM32CubeF0 "STM32CubeF0")
* [STM32L031K6T6](https://github.com/STMicroelectronics/STM32CubeL0 "STM32CubeL0")

Consult the **boot_s and vector_tables**, **device_headers**, and **ld** sections in [this guide](../../docs/import-files.md) to import the non-application files for this project.

## Procedure
The following activity is featured in this project; for the new devices:
1. Add a linker script. Pay attention to Flash and SRAM values.
2. Add a vector table. Note the names of the interrupt handlers.
3. Add device header files.
4. Update the Makefile. Remember to uncomment one default target chip.
5. Note differences in clock speed and source code implementations.

Compile the project, and flash to chip with `st-flash write main.bin 0x08000000`. Compare results. 

## Author's notes 
When I tried to flash the project to NUCLEO-L031K6 with `st-flash write main.bin 0x08000000`, it gave off errors like `Flash loader run error`. After trying to fix them, I gave up and copied the `main.elf` file to the host Windows system my Linux VM is running on, and flashed the project using **STM32CubeProgrammer**. This will be how I will flash to this Nucleo board in future attempts.
