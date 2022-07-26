# Part 9: Fun with DMA (work in progress)
This repository is a submit box for Part 9 of Vivonomicon's **"Bare Metal" STM32 Programming** blog series:

https://vivonomicon.com/2019/07/05/bare-metal-stm32-programming-part-9-dma-megamix/

## Hardware
For this lab, I used the following STM32 dev boards:

Development board | STM32F0DISCOVERY | NUCLEO-L031K6      | NUCLEO-F303K8     | STM32G0316-DISCO
------------------|------------------|--------------------|-------------------|-------------------
Microcontroller   | STM32F051R8T6    | STM32L031K6T6      | STM32F303K8T6     | STM32G031J6M6
Core              | Cortex-M0        | Cortex-M0+         | Cortex-M4         | Cortex-M0+
Flash memory      | 64 KB            | 32 KB              | 64 KB             | 32 KB
SRAM              | 8 KB             | 8 KB               | 16 KB<sup>1</sup> | 8 KB
Operating freq.   | 48 MHz           | 32 MHz<sup>2</sup> | 48 MHz            | 48 MHz
Package           | LQFP64           | LQFP32             | LQFP32            | S08N

1. 12 KB normal SRAM, 4 KB core coupled memory SRAM (CCMRAM)
2. See Author Notes

Components used for this project:
* 8 Ohm speaker (optional). Alternatively, use a scope to verify the waveform frequency.
* [WS2812B RGB LED Strip](https://www.amazon.com/gp/product/B07BKNS7DJ/ref=ppx_yo_dt_b_search_asin_title?ie=UTF8&th=1) (from Amazon)
* [SSD1306 I2C OLED display](https://www.amazon.com/gp/product/B072Q2X2LL/ref=ppx_yo_dt_b_search_asin_title?ie=UTF8&psc=1) (from Amazon)
* [ILI9341 TFT LCD display](https://www.amazon.com/gp/product/B073R7BH1B/ref=ppx_yo_dt_b_search_asin_title?ie=UTF8&psc=1) (from Amazon)
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
* Line 46: Vivonomicon toggles `done` to 0 if `ledt` is greater than `NUM_LEDS`, which is 3.
* Before `ledt` increments to 4, `grbs[3]` is read.
* `grbs` contains two elements, so `grbs[3]` is undefined.

In my case, the fourth LED shines a bright white. This issue was resolved by changing `ledt > NUM_LEDS` to `ledt >= NUM_LEDS` in line 46.

## Author's notes
As the length of this post implies, there is a better way to send the NZR data to each WS2812B LED. See my upcoming original for an example implementation. (TBA)


## For editing purposes only
The following are some of my goals involving DMA:
1. Complete VVC's tutorial as written.
   * Use devices F051, F303, L031, G031. Note that this lab will skip Type 2 DMA.
   * Note differences between them all (F0 is standard, L0 has only 8-bot or 16-bit SPI, G0 has DMAMUX)
2. Research additional example uses/features for DMA
   * Textbook DMA: DMA Chapter, DAC with DMA, I2C with DMA, SPI with DMA
   * ECE362 DMA: Cyclone lights class project. DMA homework, etc. Recall your previous analysis during Part 8 bonus.
3. Create an original project...
   * **Strobing lights** - recall your idea you wrote during Lab 8
4. Bonus 1: Investigate the G0's S08N package
   * How to access Pin 4 (PA0, PA1, PA2, **PF2-NRST**)
   * How to access Pin 8 (PB5, PB6, **PA14-BOOT0**, PA15). Note that value of BOOT0 pin during bootup is not relevant by default.
   * Motive: Use all 8 pins in package as GPIO!

Part 1: Play a Musical note
Special notes about the devices:
* Only F0 and F3 devices have a DAC.
* See the highlighted circle for a schematic of what I used to connect the 8 ohm speaker. (add image...)
  * Details: 1pF capacitor in series, between speaker and GPIO.

References: summary of DMA requests for each channel:
- F3 - See Table 78 (p. 273)
- F0 - See Table 30 (p. 202)
- L0 - See Figure 25 and Table 49 (p. 243). Note unique register DMA_CSELR.
  - Note also, that the mapping is segregated with mux, controlled by CSELR. This forces only 1 request per channel.
  - The others use OR gates, potentially accepting multiple requests per channel (is exploiting this feature a good idea?)
- G0 - See Table 52, Sect 11.3 (p. 299)

Some evidence that `DAC2_CH2` is a typo
1. DAC section in FRM only lists 1 channel for DAC2 (only in F3)
2. F3 manual, SYSCFG registers: Bit 14, `TIM7_DAC1_CH2_DMA_RMP` clearly pairs TIM7 to DAC1_CH2, not DAC2_CH2 (p. 247).
   * Note that in the register table, the shorthand for the bit name is misleading, since it reads as `TIM7_DAC2_DMA_RMP` (p. 245)
3. VS Code, stm32f303x8.h, line 9697: The macro is called `SYSCFG_CFGR1_TIM7DAC1Ch2_DMA_RMP`
4. QED.

Note the CMSIS naming conventions of DMA registers, as follows:
* `DMA1` -> `ISR`, `IFCR`. Uses `DMA_TypeDef`
* `DMA1_Channelx` -> `CCR`, `CNDTR`, `CPAR`, `CMAR`. Uses `DMA_Channel_TypeDef`
* `DMA1_CSELR` -> `CSELR`
  * Used only for L0. Uses `DMA_Request_TypeDef`
  * In C, use the line `1 << (4 * (3-1))`. This means that CSELR option 1 (SPI1) of Channel 3 is selected, which is SPI1_TX.

Special thing for G0 DMAMUX (see Sect 11.3 of G0 FRM):
* DMAMUX Ch 0 is connected to DMA Ch 1, etc.
* You want evidence? Go to `stm32g0xx.h`, lines 599-609. Just accept that DMAMUX = DMA-1.

How many DMA channels does my device have?
* F0 - 5
* L0 - 7
* F3 - 7
* G0 - 5 DMAMUX output request channels

My planned tables: One each for each project (DAC speaker, RGBLED lights, I2C OLED, TFT)
Table layout: GPIO pin | DMA request name (ex. DAC1_CH1) | alt function no. for F0 (if eligible) | .. for L0 | .. for F3 | .. for G0

DAC pin assignments for DAC speaker:

Pin function | GPIO pin | F0                   | L0 | F3                    | G0
-------------|----------|----------------------|----|-----------------------|--------
DAC1_CH1     | PA4      | DAC_OUT1<sup>1</sup> | *  | DAC1_OUT1<sup>1</sup> | *
1. Additional function (set `MODER` bits to 11)

Pin assignments for RGBLED lights:

Pin function | GPIO pin | F0  | L0  | F3  | G0
-------------|----------|-----|-----|-----|--------
SPI1_MOSI    | PB5      | AF0 | AF0 | AF5 | AF0

Pin assignments for SSD1306 I2C OLED display:

Pin function | GPIO pin | F0  | L0  | F3  | G0
-------------|----------|-----|-----|-----|--------
I2C1_SCL     | PB6      | AF1 | AF1 | AF4 | AF6
I2C1_SDA     | PB7      | AF1 | AF1 | AF4 | AF6

Pin assignments for ILI9341 TFT LCD display

Pin function          | GPIO pin               | F0  | L0    | F3  | G0
----------------------|------------------------|-----|-------|-----|----
SPI1_SCK (SPI2_SCK)   | PB3 (PB8)              | AF0 | AF0   | AF3 | (AF1)
SPI1_MOSI (SPI2_MOSI) | PB5 (PB10<sup>1</sup>) | AF0 | AF0   | AF3 | (AF0)
CS                    | PA15 (PA0)             | OUT | (OUT) | OUT | (OUT)
RESET                 | PA13 (PA1)             | OUT | (OUT) | OUT | (OUT)
DC                    | PA11 (PA3)             | OUT | (OUT) | OUT | (OUT)
1. Needs remapping of pin 6 with SYSCFG_CFGR1 register from PA12 to PA10

The DMA channel mappings for each peripheral:

DMA request | Channel # (G0 MUX input)
------------|------------------------
DAC1_CH1    | Ch3
SPI1_TX     | Ch3 (17)
SPI2_TX     | (19)
I2C1_TX     | Ch2 (11)

**Explain how you determined the settings for the waveform.**
First off, it is not recommended to run 90 LEDs from power provided through ST-LINK connector. The lights would start flickering. 3 LEDs are sufficient to demonstrate the rainbow effect.
Secondly, The definitions for the NZR bits '0' and '1', as provided by VVC, can be improved on. Note that L0 gets a unique setting, due to its clock only going to 32 MHz, compare to 48 MHz which the other MCUs were set.

The following assumptions were made:
- 8 bit SPI is used.
- The SPI clock was reduced eightfold, from 48 MHz to 6 MHz (or 4 MHz, for L0)
- This gives the sum of HIGH and LOW portions of either bit as 1333ns (2000ns).
  - Note that the WS2812 doc has the data transfer time as 1250ns.
  - But as we saw in Part 8, the LOW portion need not be adhered so strictly, and may exceed its expected time. I assume the WS2812 electronics interpret this overtime as the start of a RESET LOW pulse. The RESET pulse is cut short, maintaining the illusion that these are just '0' and '1' bits.
- Also note that the reason I made the change is because I had to adhere to the 150ns tolerances of the HIGH portions of either bit value.

Below are the measured lengths of each NZR bit value: expected (deviance). Note that actual measured values are slightly lower than the expected values, down to 50ns less than expected. Note that larger deviance is worse:

Origin                   | '0' HIGH length (ns) | '1' HIGH length (ns)
-------------------------|----------------------|---------------------
Docs                     | 400                  | 800
Vivonomicon (0xC0, 0xFC) | 333 (-66)            | 1000 (+200)
My addition (0xE0, 0xF8) | 500 (+100)           | 833 (+33)
L0-only     (0xC0, 0xE0) | 500 (+100)           | 750 (-50)

Also of note is the length of the RESET pulse. In my case, what the doc said was the minimum length was not accurate to what I eventually measured the hard way. This issue plauged my efforts to get the lights to work. So I write this down so you don't share the same fate: (adjust the length of 0x0 padded to the end for DMA to read. (NZR_RESET)). I was able to determine the actual minimum by narrowing down in a binary fashion:

Origin                   | NZR_RESET | Min RESET length (us) | Result
-------------------------|-----------|-----------------------|---------------------
Docs                     | N/A       | 50                    | Did not work
Test 2                   | 192       | 260                   | Worked (use as the safe option)
Test 3                   | 170       | 226                   | Worked (absolute minimum, dependent on device and SPI config)
Test 4                   | 169       | 225                   | Worked, but 4 lights appeared instead of 3
Test 5                   | 168       | 224                   | Worked, but 6 lights appeared instead of 3
Test 6                   | 160       | 212                   | Did not work

If you wish to avoid doing the testing procedure, I recommend you use the safe option. If you are time-constrained, test your own device by going down gradually.

Next: Show how you calculated the correct pairing for '0' and '1' bits.
In 6 MHz, each SPI bit transferred is 166.6ns long, to a total of 1333ns for all 8 of them. For L0 at 4 MHz, each SPI bit is 250ns long, to a total of 2000ns. Both are above the nominal 1250 total data transfer time listed in the docs.
