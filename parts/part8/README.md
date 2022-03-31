# Part 8: Learn to Debug Timing Issues with NeoPixels
This repository is a submit box for Part 8 of Vivonomicon's **"Bare Metal" STM32 Programming** blog series:

https://vivonomicon.com/2018/12/28/bare-metal-stm32-programming-part-8-learn-to-debug-timing-issues-with-neopixels/

## Hardware
For this lab, I used the STM32F0 Discovery board:

Development board | STM32F0DISCOVERY
------------------|-----------------
Microcontroller   | STM32F051R8T6  
Core              | Cortex-M0   
Flash memory      | 64 KB  
SRAM              | 8 KB       
Max frequency     | 48 MHz<sup>1</sup>
Package           | LQFP64   

1. In this lab, Vivinomicon sets up the clock to run at 64MHz. I don't recommend trying this for specialized peripherals, like SPI, I2C, or UART.

Components used for this project:
* [WS2812B RGB LED Strip](https://www.amazon.com/gp/product/B07BKNS7DJ/ref=ppx_yo_dt_b_search_asin_title?ie=UTF8&th=1) (from Amazon)
* Oscilloscope. Alternatively, you may use an 8-channel digital logic analyzer.

## Software
Consult the **boot_s and vector_tables**, **device_headers**, and **ld** sections in [this guide](../../import-files.md) to import the non-application files for this project.

## Procedure
The following activity is featured in this project:
1. Understand how WS2812B interprets a stream of data.
2. Understand how to send each bit of color.
3. Observe the waveform.

Compile the project, and flash to chip with `st-flash write main.bin 0x08000000`. Compare results.

### Understand how WS2812B interprets a stream of data
See the WS2812B datasheet for the following notes.

**Data transmission method**
* After reset, the first LED (connected directly to the STM32) captures the first 24 bits and sets the RGB brightness levels accordingly.
* The LED forwards all other bits to the next LED. That LED will capture the first 24 bits it recieves, and will forward the rest to the LED after that one. And so on...
* Every LED maintains the RGB brightness levels set by the first 24 bits recieved by each, until next reset.

### Understand how to send each bit of color
To manage the stream of data, Vivonomicon uses five global variables:
* `uint32_t grbs[NUM_LEDS]` - Each LED is assigned the lower 24 bits to store GRB data.
* `int ledt = 0` - Tracks the current LED
* `int ledb = 0` - Tracks the `(23-ledb)`th bit read from an LED's register.
* `int ledi = 0` - Same as `ledb`. Used after the whole stream is sent.
* `char done = 0` - Changes to `1` when all LEDs have been updated. 

The simple process (takes place in `next_pulse()`:
1. Set a timer to wait for X ticks. (X will be lower for a `0`, higher for a `1`.)
   * CNT increments every clock cycle (PSC = 0). CNT initialized to 0.
   * `tval` set to 1 or 20, depending on bit `23-ledb` from `grbs[ledt]`.
2. Pull the GPIO pin high.
3. Start the timer.
   * CNT begins incrementing.
4. Wait for the timer's counter to be >= X.
5. Pull the GPIO pin low.
   * Disable TIM16. Reset CNT to 0. 
6. Move on with the next bit.
   * `ledb` increments. Read from bit 23 to bit 0.
   * If `ledb` wraps around, `ledt` increments. Write for next LED.
   * If `ledt` wraps around, `done` changes to 1. Exit `next_pulse()`.

For the next stream, edit `grbs` to modify each LED's RGB value.

### Observe the waveform
Read the input data pin (pink on the images). Shown below, the `0` bit has a high voltage time of about 500 ns, while the `1` bit has a time of around 750 ns (both within 150 ns tolerance).

In `global.h`, I decided to modify `MPX_P0` to any number between 1 and 20. This is what I found:
* 1 to 15 : `0` bit high time remained 500 ns
* 16+ : `0` bit indistinguishable from `1` bit

My guess: Checking CNT in the while loop (tim.c, line 38) takes many cycles. It might only be checked once, so modifying `NPX_P0` to between 1 and 15 did not matter. Over 16, the loop checked again. Still, how the instructions between Line 36 (enable timer) and Line 38 (while loop) are implemented can have a big difference in timing.

### Correction 1: White LED
In `tim.c`, in `next_pulse()`.
* Vivonomicon toggles `done` to 0 if `ledt` is greater than `NUM_LEDS`, which is 3.
* Before `ledt` increments to 4, `grbs[3]` is read.
* `grbs` contains two elements, so `grbs[3]` is undefined.

In my case, the fourth LED shines a bright white. This issue was resolved by changing `>` to `>=`.

## Author's notes
As the length of this post implies, there is a better way to send the NZR data to each WS2812B LED. See my upcoming original for an example implementation. (TBA)
