# Part 6: Multitasking with FreeRTOS
This repository is a submit box for Part 6 of Vivonomicon's **"Bare Metal" STM32 Programming** blog series:

https://vivonomicon.com/2018/08/23/bare-metal-stm32-programming-part-6-multitasking-with-freertos/

## Hardware
For this lab, I used both the STM32F0 Discovery board and F303K8 Nucleo board; other boards are mentioned in the blog:

Development board | STM32F0DISCOVERY | NUCLEO-F303K8
------------------|------------------|---------------
Microcontroller   | STM32F051R8T6    | STM32F303K8T6
Core              | Cortex-M0        | Cortex-M4
Flash memory      | 64 KB            | 64 KB
SRAM              | 8 KB             | 16 KB<sup>1</sup>
Max frequency     | 48 MHz           | 72 MHz<sup>2</sup>
Package           | LQFP64           | LQFP32

1. 12 KB normal SRAM, 4 KB core coupled memory SRAM (CCMRAM)
2. Due to limits to PLL clock multiplication (see author note), this lab will run on 8 MHz instead.

## Software
Get the latest MCU Firmware Package for each supported chip here:
* [STM32F051R8T6](https://github.com/STMicroelectronics/STM32CubeF0 "STM32CubeF0")
* [STM32F303K8T6](https://github.com/STMicroelectronics/STM32CubeF3 "STM32CubeF3")

Get the latest FreeRTOS Kernel here:
* [FreeRTOS-Kernel](https://github.com/FreeRTOS/FreeRTOS-Kernel "FreeRTOS kernel only")

### Recall: how to get the files you need
See the header file dependency graphs to identify the files needed. Using STM32F0 as an example:
* To find `cmsis` and `core` files, go to
  * `STM32CubeF0/Drivers/CMSIS/Core/Include/`
* To find `stm32` files, go to
  * `STM32CubeF0/Drivers/CMSIS/Device/ST/STM32F0xx/Include/`
* To tailor your `vector_table` and `core` files for your chip, find its startup file in 
  * `STM32CubeF0/Drivers/CMSIS/Device/ST/STM32F0xx/Source/Templates/gcc/`

## Procedure
The following activity is featured in this project:
1. Setup GPIO pins to use the SPI peripheral.
2. Initialize the SPI peripheral.
3. Undertanding `sspi.c / `sspi.h` functions` used to send data. Compare Vivonomicon's implimentation with other authors.
4. Connecting the display to the microcontroller.
5. Programming the display.

Additional notes for each step is listed below:

### Setup GPIO pins to use the SPI peripheral
For PA_CS and PA_RST, consider using pins PA4 and PA5 instead, since PA12 and PA15 are not available for NUCLEO_L031K6.

### Initialize the SPI peripheral
This lab turns off the STM32's CS ('Chip Select') signal, instead using a software CS. This is a departure from my previous experience using SPI devices, in which I would use NSS.

### Undertanding `sspi.c / `sspi.h` functions` used to send data
This section is straightforward. The only thing of note is the importance of casting the Data Register `SPI1->DR`:
* For 8-bit data transfers, cast it as `*(uint8_t*)&(SPIx->DR) = dat;`
* For 16-bit data transfers, cast it as `*(uint16_t*)&(SPIx->DR) = dat;`

Note: So far, only F0-class devices can cast Data Register to 16-bit. It might be possible to cast L0-class devices to 16-bit, but I haven't tried it yet.

## Connecting the display to the microcontroller
Straightforward. Remember to use PA4 and PA5 instead of PA12 and PA15 to be able to reuse on NUCLEO-L031K6!

## Programming the display
Straightforward. In `ili9341_hspi_init()`, the SPI data transfers before PWCTR1, such as the command `0xEF`, cannot be found in the ILI9341 command list. Apparently, those are commands Adafruit acquired from another source when making their module, the meanings of which are lost to time.

## Author's notes
