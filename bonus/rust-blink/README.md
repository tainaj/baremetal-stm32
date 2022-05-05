# Bonus 4: Rust Blink
This repository is a mini-project based on Vivonomicon's exploration of "Embedded Rust":

https://vivonomicon.com/2019/05/23/hello-rust-blinking-leds-in-a-new-language/

The source code for `main.rs` is ported from a STM32F4 demo:

https://github.com/adamgreig/stm32f4-demo

## Source material
Consult the following resources referenced by Vivonomicon and this guide.
* ["Embedded Rust" ebook](https://docs.rust-embedded.org/book/ "Embedded Rust ebook") - The first two chapters cover intallation and a step-by-step guide to setup your work environment to compile a Rust project.
* ["Discovery" ebook](https://docs.rust-embedded.org/discovery/ "Discovery ebook") - Written after Vivonomicon's post. Beginner-friendly examples of embedded Rust programs, which expand the concepts demonstrated here.
* ["Rust" ebook](https://doc.rust-lang.org/book/ "Rust ebook") - An introductory book about Rust, a programming language that gives you the option to control low-level details (such as memory usage) without all the hassle traditionally associated with such control.
* [Docs.rs](https://docs.rs/) - Look up entries for "svd2rust". Note that `stm32l0x1` here (a submodule of crate `stm32l0`) is the **crates.io** version; see **Key differences** below for a walkthrough between the differences between that module and the local library crate used in this project.

## Hardware specifications
For this mini-project, I used the Nucleo L031K6 board:

Development board     | NUCLEO-L031K6
----------------------|------------------
Microcontroller       | STM32L031K6T6
Core                  | Cortex-M0+
`target` architecture | `thumbv6m-none-eabi`
Flash memory          | 32 KB
SRAM                  | 8 KB
Operating freq.       | 2.1 MHz (default)

## Create your own Peripheral Access Crate (PAC)

Otherwise known as "Make your own PAC vs use the one from crates.io"

For educational purposes, this mini-project constructs the PAC (see Chapter 2.3 of Embedded Rust) for `stm32l0x1` directly from its SVD file from STMicroelectronics. You are free to use the `stm32l0x1` submodule from **crates.io** crate `stm32l0`; see section **Key differences** for a comparison.

Remember to install the utilities required to process the SVD with the following commands: 
```
rustup component add rustfmt
cargo install svd2rust form
```

## Work in progress: A list of stuff I am writing from my written notes
Here is a list of stuff I am writing, with no immediate way to organize it yet. I'll move it to its own section once I know what to do with it.

**Some special notes to put in Procedure!**

Something else I have learned the hard way... do not compile your project with `cargo build` or `cargo check` without the following line in `main.rs`:
```rs
use stm32l0x1::{self, interrupt};
```
Without this line, I kept getting the error that the interrupt vectors are missing. So be sure to have this line available when using `cargo build`, even when you have no main code yet.

**Additional notes on the commends in main.rs**
Let me help you go through my comments
* Includes (lines 7-10) - `hprint()`, and its derivatives, is a handy tool to recieve prompt feedback when running gdb with semihosting enabled.
* Peripheral references (lines 17-22) - The variable `device_peripherals` has taken ownership of the peripherals from the crate `stm32l0x1`. `gpiob`, `rcc`, and `tim2` are just aquiring license to use them, with usage of & to make it obvious. Mutables (as used by VVC) are not necessary.
* Enable the GPIOB clock (lines 24-28) - use `modify` for most of your peripheral bit needs. Check what methods are available from your struct to use. Look at several associated `.rs` files, as shown below in this example for local crate: (It will be more obvious if you path-trace with VS Code)
* Set PB3 to be a push-pull output (lines 30-34) - `bits()` is unsafe in local crate only.
* Timer interrupts every second (lines 36-45) - `w.arr().bits()` only safe in crates.io (line 43). Also, TIM2 of local crate weirdly has `arr_l` and `arr_h` instead of `arr` (src/tim2/arr.rs, lines 94, 99, 106, 111)

See Line ? for below:
```rs
gpiob.otyper.modify(|_, w| w.ot3().clear_bit());
```
* gpiob -> Peripheral, a PTR member of struct `device_peripherals.GPIOB`. (see local crate, src/lib.rs, line 305)
* otyper -> Peripheral register, a `RegisterBlock` member of GPIOB (see src/gpiob.rs, line 7)
* modify() -> Modify current register values, a method for any implementation of `crate::Reg`, of which `otyper` is an example (see src/generic.rs, p. 147)
* w -> Field writer for implementation W (see src/gpiob/otyper.rs, line 711)
* ot3() -> OT3 method of implementation W (otyper.rs, line 774) -> returns `OT3_W` (otyper.rs, line 497)
* clear_bit() -> Derivative method of `bit()`, of `OT3_W` struct (otyper.rs, line 508)

See line 76 for a read-only example:
```rs
if (*ptr).odr.read().od3().bit() {
```
* (\*ptr) -> Dereference of `stm32l0x1::GPIOB::ptr()`.
* otyper -> Peripheral register, a `RegisterBlock` member of GPIOB (see src/gpiob.rs, line 7)
* read() -> Read current register values, a method for any implementation of `crate::Reg`, of which `otyper` is an example (see src/generic.rs, line 63)
* od3() -> Base OT3 method of implementation R (src/gpiob/odr.rs, line 692) -> returns `OD3_R` (src/gpiob/odr.rs, line 483)
* bit() -> Base method of a `Deref` for `OD3_R`, with target `FieldReader` (src/generic.rs, line 247)

The other lines are very similar. Some special notes for some registers and fields:
* Fields of every peripheral register of the local crate `stm32l0x1` all contain the base methods `bit()` and/or `bits()`, which may or may not be unsafe. Some fields may contain syntactic sugar, like `set_bit()` or `clear_bit()`, which call these base methods.
* In addition to the kind of methods described above, peripheral register fields in the crates.io module `stm32l0::stm32l0x1` have additional methods. Most of them are syntax sugar, like `push_pull()` and `open_drain()` for struct `OT3_W`. These all rely on calling the `variant()`, which also call the base methods `bit()` or `bits()`.
* Some actions (like toggling a bit field) require reading the bit field first (`|r, w|`). Most don't, though (`|_, w|`).
* Some method calls are unsafe doe to accessing critical sections (ex. `NVIC::unmask()` (line 48))
* In this example, every peripheral access is unsafe in TIM2's ISR due to their ownership asserted in main (by `device_peripherals`). These calls can be made safe, but that is beyond the scope of this post. Consult the Concurrency chapter of the Embedded Rust ebook for more.
* There are two methods to get the PTR member of a peripheral struct `GPIOB`: `GPIOB::ptr()`, and `&(Perpherals::take()).GPIOB`

Development board | crates.io    | gsdg
------------------|-------------------|--------------
Microcontroller   | STM32L031K6T6     | d
Core              | Cortex-M0+        | d
Flash memory      | 32 KB             | d
SRAM              | 8 KB              | d
Operating freq.   | 2.1 MHz (default) | d

There are several differences between the local `stm32l0x1` crate you create, and the `stm32l0::stm32l0x1` module from crates.io.
* Fields of every peripheral register of the local crate `stm32l0x1` all contain the base methods `bit()` and/or `bits()`, which may or may not be unsafe. Some fields may contain syntactic sugar, like `set_bit()` or `clear_bit()`, which call these base methods.
* In addition to the kind of methods described above, peripheral register fields in the crates.io module `stm32l0::stm32l0x1` have additional methods. Most of them are syntax sugar, like `push_pull()` and `open_drain()` for struct `OT3_W`. These all rely on calling the `variant()`, which also call the base methods `bit()` or `bits()`.
* Now for some examples!
* `w.iopben().enabled()` only availbale for crates.io (main.rs, line 27).
* `w.mode3().bits()` only safe in crates.io (line 31).
* `w.psc().bits()` only safe in crates.io (line 42).
* `w.arr().bits()` only safe in crates.io (line 43). Also, TIM2 of local crate weirdly has `arr_l` and `arr_h` instead of `arr` (src/tim2/arr.rs, lines 94, 99, 106, 111)

Final notes
* adamgreen example uses `odr3()` in src/main.rs. Both current crates.io and local version use `od3()`.
* For some reason, `cargo run` did not work. I entered `st-info --probe`, and it revealed that the stats for my board were flushed somehow (0x0, chip-id=0x0, etc.). I fixed it by calling `st-flash --connect-under-reset --reset erase`, restoring the stats.
* For your sudo needs, call `sudo apt` instead of `sudo apt-get` (as of .


## Procedure
The following activity is featured in this project:
1. Connecting VCC, GND, SCL, and SDA pins of the DS3231 module to the microcontroller.
2. Configuring GPIO pins B6 and B7 to use the I2C1 peripheral.
3. Understanding `util.c` / `util.h` functions. Compare Vivonomicon's implimentation with other authors.
4. Testing 'optional' features, including EEPROM write and read.

Additional notes for each step is listed below:

## Author's notes 
Reference material
* [Vivonomicon](https://vivonomicon.com/2018/04/30/when-is-now-the-ds3231-real-time-clock/)
* [Textbook, sect. 22.2](https://www.amazon.com/gp/product/0982692668/ref=ppx_yo_dt_b_search_asin_title?ie=UTF8&psc=1) - Zhu, Y. (2018). *Embedded Systems with ARM Cortex-M Microcontrollers in Assembly Language and C* (3rd ed.). E-Man Press LLC.
* [Pdf](../../docs/i2c-interface-ece362-engineering.purdue.edu.pdf "I2C interface")

