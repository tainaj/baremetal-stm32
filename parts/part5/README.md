# Part 5: Timer Peripherals and the System Clock
This repository is a submit box for Part 5 of Vivonomicon's **"Bare Metal" STM32 Programming** blog series:

https://vivonomicon.com/2018/05/20/bare-metal-stm32-programming-part-5-timer-peripherals-and-the-system-clock/

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
The following activity is featured in this project:
1. Optimize flash memory latency with the FLASH register.
2. Initializing and enabling the PLL clock with the RCC register. Future STM32 chips may use different configurations to accomodate different max speeds.
3. Set up TIM2 basic timer peripheral to toggle an LED every second.

## Author's notes 
Straightforward
