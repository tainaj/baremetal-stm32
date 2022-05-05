# Bonus 4: Rust Blink
This repository is a mini-project based on Vivonomicon's exploration of "Embedded Rust":

https://vivonomicon.com/2019/05/23/hello-rust-blinking-leds-in-a-new-language/

The source code for `main.rs` is ported from a STM32F4 demo:

https://github.com/adamgreig/stm32f4-demo

## Source material
Consult the following resources referenced by Vivonomicon and this guide.
* ["Embedded Rust" ebook](https://docs.rust-embedded.org/book/ "Embedded Rust ebook") - The first two chapters cover intallation and a step-by-step guide to setup your work environment to compile a Rust project.
* ["Discovery" ebook](https://docs.rust-embedded.org/discovery/ "Discovery ebook") - Written after Vivonomicon's "Rust Blink" post. Beginner-friendly examples of embedded Rust programs, which expand the concepts demonstrated here.
* ["Rust" ebook](https://doc.rust-lang.org/book/ "Rust ebook") - An introductory book about Rust, a programming language that gives you the option to control low-level details (such as memory usage) without all the hassle traditionally associated with such control.
* [Docs.rs](https://docs.rs/) - Look up entries for "svd2rust". Note that `stm32l0x1` here (a submodule of crate `stm32l0`) is the **crates.io** version; see **Note 3** below for a few examples of two equivalent lines of code, one adhering to **crates.io**, the other adhering to the local PAC's source code.

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

## Software requirements
For projects programmed with Embedded Rust, non-application files present in other posts, such as those from **boot_s** **vector_tables**, **device_headers**, and **ld** sections, will not be imported.

Consult Chapter 1 of Embedded Rust for an installation guide. It is highly recommended to try some examples from the official documentation first; check out [my written guide](../../docs/embedded-rust.md) for a full walkthrough. Note that this coincides with Step 1: Toolchain Setup from Vivinomicon's post.

## Procedure
The following activity goes step-by-step with Vivinomicon's section names:
1. Install Rust and the Embedded Rust toolchain. Practice by building example code from the ebook documentation.
2. Generate a Peripheral Access Crate (PAC). Acquire the SVD file for your board from STMicroelectronics.
3. Setup an Embedded Rust application with `cargo-generate`. Include your recently created PAC as a dependency.
4. Write a blinking-LED program with interrupts. Refer to **adamgreig**'s STM32F4 demo for the coding style.
5. Build, flash, run the project. Self-explanatory.

Additional notes for each step is listed below:

### Step 2, Note 1 - Generating your own PAC vs using the PAC from crates.io 
For educational purposes, this mini-project constructs the PAC (see Chapter 2.3 of Embedded Rust) for `stm32l0x1` directly from its SVD file from STMicroelectronics. You are free to use the `stm32l0x1` submodule from **crates.io** crate `stm32l0` for your own projects. The submodule contains additional functions to make the code more readable; see section **Key differences?** for more.

Remember to install the utilities required to process the SVD with the following commands: 
```
rustup component add rustfmt
cargo install svd2rust form
```

### Step 3, Note 2 - Before you compile your project...
Something else I have learned the hard way... do not compile your project without the following line in `main.rs`:
```rs
use stm32l0x1::{self, interrupt};
```
This is caused by the `"rt"` feature in your PAC overriding the ARM Cortex-M interrupt table with the one defined in the `stm32l0x1` crate, which must be included in `main.rs`. Without this line, Rust will complain that the interrupt vectors are missing. So be sure to have this line available when using `cargo build` or `cargo check`, even when you have no main code yet.

### Step 4, Note 3 - Commented-out lines
Certain lines in `main.rs` are commented out. Most of these use alternate syntax that do the same functionality as a nearby uncommented line.

**Includes (lines 7-10)**

`hprint()`, and its derivatives, is a handy tool to recieve prompt feedback when running `gdb-multiarch` with semihosting enabled (lines 8, 83).

**Peripheral references (lines 14-22)**

The variable `device_peripherals` has taken ownership of the peripherals from the crate `stm32l0x1` (line 15). `gpiob`, for example, just acquired a reference to use them, with usage of & to make it obvious (line 20). Mutables (as used by VVC) are not necessary (line 19).

**Enable the GPIOB clock (lines 24-28)**
Unless one is starting from reset (line 28), use `modify` for most of your peripheral register write needs (line 25). The `r` argument is optional in this case, since we are not toggling a bit, but overwriting it (line 26). Check what methods are available from your local PAC (line 27, easier if you path-trace with VS Code).

**Set PB3 to be a push-pull output (lines 30-34)**

The `bits()` method is unsafe in local PAC only (lines 31-33).

**TIM2 interrupts every second (lines 36-45)**

The `w.arr().bits()` method is safe in crates.io PAC only (line 43). Also, TIM2 of local crate weirdly has `arr_l` and `arr_h` instead of `arr` (line 44).

### Step 4, Note 4 - In-depth analysis of `read` and `write`
For reference, consult the local PAC source code. See Line 34 for a write-only example:
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
* otyper -> Peripheral register, a `RegisterBlock` member of GPIOB (see loca crate, src/gpiob.rs, line 7)
* read() -> Read current register values, a method for any implementation of `crate::Reg`, of which `otyper` is an example (see src/generic.rs, line 63)
* od3() -> Base OT3 method of implementation R (src/gpiob/odr.rs, line 692) -> returns `OD3_R` (src/gpiob/odr.rs, line 483)
* bit() -> Base method of a `Deref` for `OD3_R`, with target `FieldReader` (src/generic.rs, line 247)

### Step 4, Note 5 - Miscellaneous tips
The other lines are very similar. Some special notes for some registers and fields:
* Some actions (like toggling a bit field) require reading the bit field first (`|r, w|`). Most don't, though (`|_, w|`).
* Some method calls are unsafe doe to accessing critical sections (ex. `NVIC::unmask()` (line 48))
* In this example, every peripheral access is unsafe in TIM2's ISR due to their ownership asserted in main (by `device_peripherals`). These calls can be made safe, but that is beyond the scope of this post. Consult the Concurrency chapter of the Embedded Rust ebook for more.
* There are two methods to get the PTR member of a peripheral struct `GPIOB`: `GPIOB::ptr()`, and `&(Perpherals::take()).GPIOB`

There are several differences between the local `stm32l0x1` crate you create, and the `stm32l0::stm32l0x1` module from crates.io.
* Fields of every peripheral register of the local crate `stm32l0x1` all contain the base methods `bit()` and/or `bits()`, which may or may not be unsafe. Some fields may contain syntactic sugar, like `set_bit()` or `clear_bit()`, which call these base methods.
* In addition to the kind of methods described above, peripheral register fields in the crates.io module `stm32l0::stm32l0x1` have additional methods. Most of them are syntax sugar, like `push_pull()` and `open_drain()` for struct `OT3_W`. These all rely on calling the `variant()`, which also call the base methods `bit()` or `bits()`.

## Author's notes
* adamgreig example uses `odr3()` in src/main.rs. Both current crates.io and local version use `od3()`.
* For some reason, `cargo run` did not work. I entered `st-info --probe`, and it revealed that the stats for my board were flushed somehow (0x0, chip-id=0x0, etc.). I fixed it by calling `st-flash --connect-under-reset --reset erase`, restoring the stats.
* For your sudo needs, call `sudo apt` instead of `sudo apt-get`
