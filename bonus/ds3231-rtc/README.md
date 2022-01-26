# Part 4: Intro to Hardware Interrupts
This repository is a submit box for Part 4 of Vivonomicon's **"Bare Metal" STM32 Programming** blog series:

https://vivonomicon.com/2018/04/28/bare-metal-stm32-programming-part-4-intro-to-hardware-interrupts/

## Hardware
The blog uses a Nucleo development board for the STM32L031K6 chip. For this lab, I used both this board and the Discovery development board used in previous labs.

## Software
Firmware packages for STM32F051R8T6 and STM32L031K6T6 have been imported from the previous lab.

## Procedure
Do the following additions to your project folder, as descibed in Part 4:
* Implement interrupt handlers by creating `nvic.h`, `nvic.c`.
* Move device configuration and pin mappings to shared header `global.h`.
* Modify `main.c`  and `main.h` to add ECTI intterupts for button press and rotary encoder.

Compile a new `main.elf` file, and flash the project, as described in Part 4. Compare results.

## Author's notes 
Relatively straightforward. To add hardware debouncing, see my [bonus follow-up](../../bonus/hw-debounce/ "HW debounce") to this part.
