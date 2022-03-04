# Part 6: Multitasking with FreeRTOS
This repository is a submit box for Part 6 of Vivonomicon's **"Bare Metal" STM32 Programming** blog series:

https://vivonomicon.com/2018/08/23/bare-metal-stm32-programming-part-6-multitasking-with-freertos/

## Hardware
For this lab, I used the STM32F0 Discovery board and F303K8 Nucleo board; other boards are mentioned in the blog:

Development board | STM32F0DISCOVERY | NUCLEO-F303K8
------------------|------------------|---------------
Microcontroller   | STM32F051R8T6    | STM32F303K8T6
Core              | Cortex-M0        | Cortex-M4
Flash memory      | 64 KB            | 64 KB
SRAM              | 8 KB             | 16 KB<sup>1</sup>
Max frequency     | 48 MHz           | 72 MHz<sup>2</sup>
Package           | LQFP64           | LQFP32

1. 12 KB normal SRAM, 4 KB core coupled memory SRAM (CCMRAM)
2. Due to limits to PLL clock multiplication (see [Part 5](../part5) for clock configuration), this lab will run on 8 MHz instead.

## Software
Get the latest MCU Firmware Package for each supported chip here:
* [STM32F051R8T6](https://github.com/STMicroelectronics/STM32CubeF0 "STM32CubeF0")
* [STM32F303K8T6](https://github.com/STMicroelectronics/STM32CubeF3 "STM32CubeF3")

Get the latest FreeRTOS Kernel here (this lab uses version 202112.00):
* [FreeRTOS-Kernel](https://github.com/FreeRTOS/FreeRTOS-Kernel "FreeRTOS kernel only")

Consult the **boot_s and vector_tables**, **device_headers**, and **ld** sections in [this guide](../../import-files.md) to import the non-application files for this project. Consult the **freertos** section to import FreeRTOS files.

## Procedure
The following activity is featured in this project:
1. Understand how FreeRTOS works; it is a task scheduler.
2. Understand the project structure.
3. Tailor `FreeRTOSConfig.h` to this project.

Compile the project, and flash to chip with `st-flash write main.bin 0x08000000`. Compare results.

Additional notes for each step is listed below:

### Understand how FreeRTOS works
FreeRTOS is a task scheduler that allows an embedded device to run multiple threads while retaining the ability to react quickly to important realtime signals (taken from Vivonomicon).

FreeRTOS functions this project uses (feel free to research these on your own):
* xTaskCreate() - Creates a task for the scheduler to add, defined somewhere in the form
`void new_task(void *args)`
* vTaskStartScheduler() - Starts the scheduler; this should never return.
* vTaskDelay() - Delay a task for a given number of ticks. 

### Understand the project structure.
This part is staightforward; the only change from previous projects is the **freertos** folder and its contents.

### Tailor `FreeRTOSConfig.h` to this project
See the [FreeRTOS configuration guide](https://www.freertos.org/a00110.html) to learn about each config option in this file.

Some special notes on Cortex-M specific definitions:
* Defines for `configKERNEL_INTERRUPT_PRIORITY` and `configMAX_SYSCALL_INTERRUPT_PRIORITY` are relevant only for STM32 MCUs with a Cortex-M core that runs on ARMv7-M architecture (such as STM32F303K8).
* For Arm Cortex-M devices, the higher the priority number, the less urgency an interrupt can have. See the [following](https://community.arm.com/arm-community-blogs/b/embedded-blog/posts/cutting-through-the-confusion-with-arm-cortex-m-interrupt-priorities "ARM community blogs") [links](https://www.freertos.org/RTOS-Cortex-M3-M4.html "FreeRTOS note for Cortex-M") for more details.
  * Explains why lowest priority is not 0, but 2<sup>configPRIO_BITS</sup>.
* For Arm Cortex-M devices, an interrupt's priority occupies the most significant bits in an 8-bit register.
  * Explains why the priority value is shifted by (8 - configPRIO_BITS).
* `configASSERT(x)` is used to test for errors during development. If a variable does not match expected values, the program disables all interrupts and enters a forever loop.
  * For STM32 MCUs with ARMv7-M architecture, interrupts with urgency higher than `configMAX_SYSCALL_INTERRUPT_PRIORITY` are not disabled. To disable those also, replace the line `taskDISABLE_INTERRUPTS()` with `__disable_irq()`. More info [here](https://forums.freertos.org/t/how-to-disable-interrupt-on-cortex-m/10046/2 "FreeRTOS forum answer").

## Author's notes
Take the time to read through some FreeRTOS documentation and demos to understand how a typical FreeRTOS project is structured. Alternatively, see the [follow-up](../part6-followup) to this lab.
