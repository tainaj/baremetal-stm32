# Part 6 follow-up: FreeRTOS Demo Application
This repository is a follow-up to Part 6 of Vivonomicon's **"Bare Metal" STM32 Programming** blog series:

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
Import the code from this repository. Some key differences between this and Part 6:
* `Makefile`: This is a purely C-based FreeRTOS project; just copy the content from the latest non-C++ Vivonomicon commit.
* `FreeRTOSConfig.h`: Copied from Part 6, but with config settings from **STM32F0518_IAR demo** (see Procedure section)
* `freertos/Demo/`: contains common demo application files

## Procedure
The following activity is featured in this project:
1. Understand `Demo/` files.
2. Update Makefile to include new files.
3. Modify `main-blinky` demo to the microcontroller.
4. Modify `main-full` to the microcontroller.

Compile the project, and flash to chip with `st-flash write main.bin 0x08000000`. Compare results.

Additional notes for each step is listed below:

### Understand `Demo/` files
FreeRTOS contains [demo applications](https://www.freertos.org/a00102.html)
that test various FreeRTOS features, such as queues, mutexes, semaphores, etc.
Most demo applications contain **main_blinky.c**, which simply demonstrates how a FreeRTOS queue can be used to toggle an LED,
and **main_full.c**, which contains more comprehensive tests.

This follow-up project was originally sourced from FreeRTOS Github folder `FreeRTOS/FreeRTOS/Demo/STM32F0518_IAR`,
found [here](https://github.com/FreeRTOS/FreeRTOS/tree/main/FreeRTOS/Demo/CORTEX_M0_STM32F0518_IAR).

`ParTest` files contain functions used to interact with STM32 peripherals, such as system clocks and GPIO pins.
Do the following:
* Comment out every #include, #define, and variable declaration except `ParTest.h`
* In `vParTestInitialize()`, add code to initialize the system clock and the built-in LED.
* In `vParTestSetLED()` and `vParTestToggleLED()`, replace the `STM_EVAL` lines with their functional CMSIS equivalents (ex. GPIOx->ODR ^= (1 << LED_PIN))

### Update Makefile to include new files.
For the follow-up, I decided not to use C++ for the user application files,
and used an the last C-based commit by Vivonomicon as reference. Remember to:

* Include each header under "Common demo includes" listed in **main_full.c**.
* Be sure to include the corresponding source file, as well.

### Modify `main-blinky` demo to the microcontroller
* Rreplace `LED1` with `LED_BANK`, defined in `global.h`.


### Modify `main-full` demo to the microcontroller
* Replace `LED1` with `LED_BANK`, defined in `global.h`.
* To focus on a demo task(s), comment out other demo task sections in `main_full()` and `prvCheckTimerCallback()`.

## Author's notes
Troubleshooting:
* If you get the error R_ARM_THM_JUMP11: (undefined reference to main, due to large program size)
  * In `boot_s/STM32F051R8T6_boot.S`, line 72: Change "B main" to "BL main". More info [here](https://stackoverflow.com/questions/64835630/linker-relocation-truncated-to-fit-r-arm-thm-jump11-error-and-gcc-output-too-bi).
* If LED will not blink:
  * Debug project with `arm-none-eabi-gdb`. Check if the process entered a forever loop.
  * If the forever loop is within `vApplicationMallocFailedHook()`:
  * Increase configTOTAL_HEAP_SIZE. Be sure not to allocate more space than what exists for the STM32 MCU.
  * Or, comment out unneeded demo tests.

IF you are successful in implementing this project, the LED will either blink once every three seconds (all tests pass),
or multiple times per second (one or more tests fail).
