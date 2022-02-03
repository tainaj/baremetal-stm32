# Part 3.5: Supporting Multiple Chips
This repository is a submit box for Part 3.5 of Vivonomicon's **"Bare Metal" STM32 Programming** blog series:

https://vivonomicon.com/2018/04/25/bare-metal-stm32-programming-part-3-5-supporting-multiple-chips/

## Hardware
The blog uses a Nucleo development board for the STM32L031K6 chip. For this lab, I used both this board and the Discovery development board used in previous labs. Compare below:

STM32 chip        | STM32F031K6T6 | STM32L031K6T6
------------------|---------------|---------------
Core              | Cortex-M0     | Cortex-M0+
Flash memory      | 64 KB         | 32 KB
SRAM              | 8 KB          | 8 KB
Max frequency     | 48 MHz        | 32 MHz
Operating voltage | 2.0 - 3.6 V   | 1.8 - 3.6 V
Package           | LQFP48        | LQFP32

## Software
Links to STMicroelectronics' MCU Firmware Package for each supported chip are below:
* [STM32F051R8T6](https://github.com/STMicroelectronics/STM32CubeF0 "STM32CubeF0")
* [STM32L031K6T6](https://github.com/STMicroelectronics/STM32CubeL0 "STM32CubeL0")

To find `boot_code`, `device_headers`, and `vector_table` files, follow the guide under the **Software** header in [Part 3's README](../part3).

## Procedure
Do the following additions to your project folder, as descibed in Part 3.5:
* Modify folder structure to categorize **boot_code** files.
* Create linker script `STM32L031K6T6.ld`.
* Rename existing vector table file to `STM32F051R8T6_vt.S`. Create vector table file `STM32L031K6T6_vt.S`.
* Add required header files from STMicroelectronics.
* Update Makefile. Note the `VVC_F0` and `VVC_L0` flags added.
* Update `main` files from the previous project.

Compile a new `main.elf` file, and flash the project, as described in Part 3.5. Compare results. 

## Author's notes 
When I tried to flash the project to the Nucleo board for STM32L0 with `st-flash write main.bin 0x08000000`, it gave off errors like `Flash loader run error`. After trying to fix them, I gave up and flashed the `main.elf` file using **STM32CubeProgrammer** in Windows. This will be how I will flash to the Nucleo board in future attempts.
