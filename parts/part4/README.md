# Part 4: Intro to Hardware Interrupts
This repository is a submit box for Part 4 of Vivonomicon's **"Bare Metal" STM32 Programming** blog series:

https://vivonomicon.com/2018/04/28/bare-metal-stm32-programming-part-4-intro-to-hardware-interrupts/

## Hardware
For this lab, I used the STM32F0 Discovery board and Nucleo L031K6 board:

Development board | STM32F0DISCOVERY | NUCLEO-L031K6
------------------|------------------|---------------
Microcontroller   | STM32F051R8T6    | STM32L031K6T6
Core              | Cortex-M0        | Cortex-M0+
Flash memory      | 64 KB            | 32 KB
SRAM              | 8 KB             | 8 KB
Max frequency     | 48 MHz           | 32 MHz
Package           | LQFP64           | LQFP32

## Software
Consult the **boot_s and vector_tables**, **device_headers**, and **ld** sections in [this guide](../../docs/import-files.md) to import the non-application files for this project.

## Procedure
The following activity is featured in this project; for the new devices:
1. Implement interrupt handlers by creating `nvic.h`, `nvic.c`.
2. Move device configuration and pin mappings to shared header `global.h`.
3. Modify `main.c`  and `main.h` to add EXTI intterupts for button press and rotary encoder.

Compile the project, and flash to chip with `st-flash write main.bin 0x08000000`. Compare results.

## Author's notes 
Relatively straightforward. To add hardware debouncing, see my [follow-up](../part4-followup/ "HW debounce") to this part.
