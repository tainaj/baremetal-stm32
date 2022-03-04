# Bonus 2: ILI9341 TFT LCD display
This repository is a mini-project based on Vivonomicon's exploration of the ILI9341 TFT:

https://vivonomicon.com/2018/06/17/drawing-to-a-small-tft-display-the-ili9341-and-stm32/

## Hardware
For this mini-project, I used the STM32F0 Discovery board and Nucleo L031K6 board:

Development board | STM32F0DISCOVERY | NUCLEO-L031K6
------------------|------------------|---------------
Microcontroller   | STM32F051R8T6    | STM32L031K6T6
Core              | Cortex-M0        | Cortex-M0+
Flash memory      | 64 KB            | 32 KB
SRAM              | 8 KB             | 8 KB
Max frequency     | 48 MHz           | 32 MHz
Package           | LQFP64           | LQFP32

Components used for this project:
* [ILI9341 TFT LCD display](https://www.amazon.com/gp/product/B073R7BH1B/ref=ppx_yo_dt_b_search_asin_title?ie=UTF8&psc=1) (from Amazon)

## Software
Consult the **boot_s and vector_tables**, **device_headers**, and **ld** sections in [this guide](../../import-files.md) to import the non-application files for this project.

## Procedure
The following activity is featured in this project:
1. Setup GPIO pins to use the SPI peripheral.
2. Initialize the SPI peripheral.
3. Undertanding `sspi.c` / `sspi.h` functions used to send data. Compare Vivonomicon's implimentation with other authors.
4. Connecting the display to the microcontroller.
5. Programming the display.

Additional notes for each step is listed below:

### Setup GPIO pins to use the SPI peripheral
For PA_CS and PA_RST, consider using pins PA4 and PA5 instead, since PA12 and PA15 are not available for NUCLEO_L031K6.

### Initialize the SPI peripheral
This lab turns off the STM32's CS ('Chip Select') signal, instead using a software CS. This is a departure from my previous experience using SPI devices, in which I would use NSS.

### Undertanding `sspi.c` / `sspi.h` functions used to send data
The only thing of note is the importance of casting the Data Register `SPI1->DR`:
* For 8-bit data transfers, cast it as `*(uint8_t*)&(SPIx->DR) = dat;`
* For 16-bit data transfers, cast it as `*(uint16_t*)&(SPIx->DR) = dat;`

Note: So far, only F0-class devices can cast Data Register to 16-bit. It might be possible to cast L0-class devices to 16-bit, but I haven't tried it yet.

## Connecting the display to the microcontroller
Remember to use PA4 and PA5 instead of PA12 and PA15 to be able to reuse on NUCLEO-L031K6!

## Programming the display
In `ili9341_hspi_init()`, the SPI data transfers before PWCTR1, such as the command `0xEF`, cannot be found in the ILI9341 command list. Apparently, those are commands Adafruit acquired from another source when making their module, the meanings of which are lost to time.

## Author's notes 
Straightforward
