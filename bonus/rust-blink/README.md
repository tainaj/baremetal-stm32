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
* [Docs.rs](https://docs.rs/) - Look up entries for "svd2rust". Note that the `stm32l0x1` module here is the **crates.io** version; see **Key differences** below for a walkthrough between the differences between that module and the local library crate used in this project.

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

## Initial installation (move to its own Rust markdown file if need be)
Since this mini-project is programmed in Rust, non-application files present in other posts, such as those from **boot_s** **vector_tables**, **device_headers**, and **ld** sections, will not be imported. Use of `cargo-generate` will auto-generate these files with minor tweaking required (see Section ? for more).

### Downloaded files
The only file download is ST Microelectronics' System View Description (SVD) file for the [target board](https://www.st.com/en/microcontrollers-microprocessors/stm32l0-series.html). Go to **CAD Resources**, under **System View Description**. You may get a zip file; download and extract it. The collection may target a wider selection of boards; locate the SVD file specific to you.

### Embedded Rust toolchain
Embedded Rust is an extension of Rust on "Bare Metal" embedded systems. Begin installation by entering the following `curl` command anywhere in Bash:
```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
This will install `rustup`, which is essential to Rust programming. Read Chapters 1.3 and 1.4 from the Embedded Rust ebook for a complete overview of the following installations:

Utility   | About                                                  | Installed by               | Features
----------|--------------------------------------------------------|----------------------------|-------------------------------------------------------
`rustup`  | Installer for the systems programming language Rust    | `curl` command (see above) | <ul><li>`rustc` - Rust compiler</li><li>`cargo` - Rust package manager</li><li>Switch between `stable`/`nightly` toolchain branches</li><li>Update Rust toolchain with `rustup update`</li><li>Cross-compilation with ARM Cortex-M architectures, like `thumbv6m-none-eabi`</li></ul>
`rustc`   | Compiler for the Rust programming language             | Bundled with `rustup`      | <ul><li>Behind-the-scenes action required for `cargo`</li></ul>
`cargo`   | Rust package manager                                   | Bundled with `rustup`      | <ul><li>Project essentials: `cargo new`, `cargo build`, `cargo run`, `cargo check`, `cargo update`, `cargo clean`</li></ul>
`cargo-binutils` | LLVM tools used to inspect binaries | `cargo install` | <ul><li>Binary essentials: `cargo readobj`, `cargo size`, `cargo objdump`</li><li>See Chapter ? for more</li></ul>
`cargo-generate` | Generate a Rust project template    | `cargo install` | <ul><li>`cortex-m-quickstart` - STM32 Cortex-M template</li><li>See Chapter ? for more</li></ul>

### Other packages
The following installations are mentioned in Chapter 1.4.1 of the Embedded Rust ebook:
Utility           | About                                                  | Installed by       | Features
------------------|--------------------------------------------------------|--------------------|------------------------------
`gdb-multiarch`   | Preferred by these ebooks          | `sudo apt install` | <ul><li>same toolset as `arm-none-eabi-gdb`</li></ul>
`openocd`         | "Open On-chip debugger"                                | `sudo apt install` | <ul><li>Alternative to **st-link's** `st-util` utility</li><li>Simplified debugging with helper files `openocd.cfg` and `openocd.gdb`</li><li>**Works for NUCLEO-L031K6 dev board!**</li></ul>
`qemu-system-arm` (optional) | Open-source ARM machine emulator            | `sudo apt install` | <ul><li>Write a program for various ARM devices, such as the `LM3S6965`, without any hardware!</li></ul>

## Work in progress: A list of stuff I am writing from my written notes
Here is a list of stuff I am writing, with no immediate way to organize it yet. I'll move it to its own section once I know what to do with it.

The following commands will prepare your Rust work environment to work with the `nightly` branch, a more frequently updated branch (proof: 
```
rustup install nightly
rustup default nightly
```

The following commands add some utilities used to adapt the L0 SVD from STMicroelectronics to a Rust Peripheral Access Crate.
```
rustup component add rustfmt
cargo install svd2rust form
```

* Hint: Read through Chapter 2 of the Embedded Rust ebook.
**My recommendation (what I did) for trying Rust for the first time:**
In Rust ebook, compile and run "Hello, World" and "Hello Cargo", from Chapters 1.3 and 1.4, respectively.
New tools relevant for the main goal:
* `cargo new` -> Setup your Rust project workspace. For embedded use, replace with `cargo-generate` for your project template needs. Later used to help create your own PAC.
* `cargo build` -> Use to compile your project. Checks for syntax errors. Append with --release to compile with optimizations, to optimize space and performance. Remember to comment out all instances of `hprint()`.
* `cargo run` -> Compiles (if not already), then executes your project. Depending of the scenario, this command can do several things:
  1. Normal Rust programming -> Exactly as it says on the tin
  2. Embedded Rust programming -> Runs GDB to connect and flash the program to the STM32 board. Uses OpenOCD, with aid of helper files `openocd.cfg` and `openocd.gdb`. Be sure to run `openocd` first.
* `cargo check` -> Supposedly does what `cargo build` does, but with less time. Not in my experience, though, but perhaps it is because I like to invoke...
* `cargo update` -> Does this update the crates? And might this break the dependency chain for any local crates used by my cargo project?
* `cargo clean` -> Removes autogenerated build files from the 'target' directory.

**Interesting notes about learning Rust
* immutable variables (default) vs mutable variables
* Ownership of resources vs references
* Unsafe actions
* Char type in Rust is based on Unicode (UTF-8), wihch is 4 bytes long.

**What are packages? What are Structs? What on earth are the different kinds of subsections?**
* See Chapter 6 of Rust ebook for a better understanding of these divisions of data.
* Crate. Also top-level module. Typically represent an entire STM32 family. Example:
  * `stm32l0` (STM32L0 MCU family)
* Module. May contain submodules, structs, and other stuff. Examples:
  * `stm32l0::stm32l0x1` (STM32L0x1 subfamily, which includes STM32L031K6)
  * `stm32l0:stm32l0x1::gpioa` (Module-form of GPIOA peripheral)
* Struct. Heterogeneous collection of other types, called *fields*. Contain methods that return data values, like primitives or Structs. Examples:
  * `let device_peripherals = stm32l0x1::Peripherals::take().unwrap();` (The struct `stm32l0x1::Peripherals` has a method chain `take().unwrap()` that transfers ownership of all peripheral modules from crate `stm32l0x1` to a new variable `device_peripherals`.)
  * `let ptr = stm32l0x1::GPIOB::ptr();` (The struct `stm32l0x1::GPIOB` has a method `ptr()` that licenses permission to borrow the GPIOB module to a new variable `ptr`. Remember to dereference `ptr` when calling for struct members!)

1. Methods are generally associated with each other through various implementation types, like the Struct itself.

**Chapter 2.2 - Hardware**
This command will generate a "Hello, world" template for a Cortex-M quick start:
```
cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart
```
Note that the template is configured for `thumbv7m`, an ARM architecture for a Cortex-M3 MCU.
To tailor your project to you NUCLEO-L031K6 development board (see **Hardware** for device stats), make the following changes:
* .cargo/config.toml -> `target = "thumbv6m-none-eabi"`, `runner = "gdb-multiarch -q -x openocd.gdb"`, `"-C", "linker=arm-none-eabi-ld"` (only if `cargo build` does not work despite you doing the right stuff)
* memory.x -> `FLASH : ORIGIN = 0x08000000, LENGTH = 32K`, `RAM : ORIGIN = 0x20000000, LENGTH = 8K`
* examples/ -> `rm -rf examples/` (we will not use the examples thing...)

**How openocd works in our work environment**
Let's understand the following command, shall we?
```
openocd -f interface/stlink-v2-1.cfg -f target/stm32l0.cfg
```
This command will *initiate a connection with the ST-LINK interface for the STM32L0 device in TCP port 3333*. Be sure to open a new terminal to do other tasks, as this command will *brick your current terminal*.

You may type the above command in its entirety, if you wish. But a shortcut would be to simply type:
```
openocd
```
This is made possible by the helper file `openocd.cfg`, which is used by openocd by default, if it exists. By the way, the locations for `interface/stlink-v2-1.cfg` and `target/stm32l0.cfg` are found in this directory:
```
/usr/share/openocd/scripts/
```
Again, by the way, `target/stm32l0.cfg` was an educated guess. `interface/stlink-v2-1.cfg`, on the other hand, required guess and check. Supposedly, the Rust ebook suggested `interface/stlink.cfg`, but I guess that was not available for me in the source directory.

**How gdb-multiarch works in our work environment**
Remember to call `openocd` first, and then open a new terminal to complete this step. You may use any gdb that works for embeddded device (I use `gdb-multiarch`):
```
<gdb> -q target/thumbv6m-none-eabi/debug/examples/hello
```
Once you are in gdb, type the following commands:
```
target remote :3333
load
monitor arm semihosting enable
break main
```
The executable that is loaded with `load` is provided by its name after the q. At this point, you may enter `continue` several times until you reach the breakpoint in front of `#[entry]`. After this, entering `continue` once more will start the program.

For a shortcut, just type:
```
cargo run
```
This is made possible by the file `.cargo/config.toml`, which specifies a `runner` that specifies `runner = "gdb-multiarch -q -x openocd.gdb"`. The gdb tool used and a call to `openocd.gdb` which supplies all the above GDB commands. At this 

By the way, both openocd.cfg and openocd.gdb are auto-generated by `cargo generate`.

*Note: There were not enough hardware breakpoints for my STM32 device (only two), so I commented out DefaultHandler (line 10 of gdb file) and HardFault (line 11).*

**Some special notes to put in Procedure!**
This is the list of steps I did to get my example to work:
* Download STM32L0 SVD (mine was called `en.stm32l0_svd`). Open link to STM32L0 series from ST Micorelectronics. Go to **CAD Resources**. Look under **System View Description**. You will get a zip file. Extract it.
* Locate `STM32L0_svd_<version>/STM32L0x1.svd` in the extracted folder.

Somewhere else, create a new *library crate*, copy the SVD file to it, and remove `src/` folder
* `cargo new stm32l0x1 --lib --name stm32l0x1`
* `cd stm32l0x1/`
* `cp <SVD location>/STM32L0x1.svd .`
* `rm -rf src/`

Follow the instructions on docs.rs to convert your SVD file into a Rust crate using `svd2rust`. Some notes to remember:
* Check what the documentation says are the current dependencies and what versions they require.
* Since we are on the nightly branch, run `--nightly`.
* After running `svd2rust`, edit the MEMORY section of device.x to match Flash and RAM size of your device (32K, 8K).
* To make sure that the crate will build, you may run `cargo build --target thumbv6m-none-eabi`. If there are no issues, this local `stm32l0x1` library crate is ready for the next step!

Now, you may go to wherever your baremetal STM32 bonus projects are kept, and generate a new Cortex-M project template (`cargo generate...`). Be sure to add your local crate in your project's dependencies on Cargo.toml. Include the path to your crate. For instance, I moved my local crate inside my project directory so the results are a little more obvious.

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

### Connecting DS3231 pins to STM32
The blog mentions the use of a disk battery to provide backup power to the module. Some modules, like mine, can run entirely on microcontroller power and not even need the battery. The module I linked in **Hardware** is one of those modules.

### Configuring pins to I2C1
The source material enabled internal pull-ups (25-55kOhms) for SCL and SDA lines, which are adequate for standard I2C clock speeds. For higher speeds, use external pull-ups with lower resistance (1-4.7kOhms) to reduce the rise time of the I2C lines.

This project's I2C clock speed is 100kHz, considered standard speed.

### Understanding util.c and util.h
Compare Vivinomicon's implementation of the following I2C functions with other authors:

**i2c_init()** (written in main.c)

VVC wrote this function's logic as part of `main`. GPIOB pins B6 and B7 are enabled, set to I2C1. Bits in `I2C->CR1`, `I2C->CR2`, and `I2C->ISR` are cleared to return registers to their reset value 0x00000000.

The pdf and the textbook also set `AUTOEND` and `NACK` in `I2C1->CR2`:
* `AUTOEND`: STOP condition automatically set after `NBYTES` data sent. May make the setting of `I2C_CR2_STOP` in `i2c_stop()` redundant.
* `NACK`: This was added as a catch-all for any failure of an I2C write transfer. See `i2c_senddata()` on the pdf to see its purpose.

**i2c_start()**

VVC simply sets `I2C_CR2_START` and waits for it to acknowledge:
```c
// Send 'Start' condition, and wait for acknowledge.
I2C1->CR2 |=  (I2C_CR2_START);
while ((I2C1->CR2 & I2C_CR2_START)) {}
```
Note: `START` auto-clears when the START condition is generated on the I2C lines. Not sure if we have to wait for that to happen before writing a byte, but let's keep it to be on the safe side.

### i2c_write_byte() and i2c_read_byte()
VVC reads and writes data one byte for each function call. After writing the byte to `I2C1->TXDR`, it checks the `I2C1->ISR` register to see if `TXIS` (ready for next byte) or `TC` (transfer complete) sets. The helper function `i2c_read_register()` reads a byte from an address `reg_addr` by first writing 0 bytes while also moving the read pointer to the desired address, and then...

## Testing optional features
After writing 'Hello, World!' to the EEPROM, reading the EEPROM should return the same string.

In `gdb-multiarch`, to read the value of `eeprom_str` containing the string read, you must load and let continue the program for some time before hitting `CTRL-C`, and then entering `u` several times until you reach the `main` stackframe, in which the variable `eeprom_str` is stored.

## Author's notes 
Reference material
* [Vivonomicon](https://vivonomicon.com/2018/04/30/when-is-now-the-ds3231-real-time-clock/)
* [Textbook, sect. 22.2](https://www.amazon.com/gp/product/0982692668/ref=ppx_yo_dt_b_search_asin_title?ie=UTF8&psc=1) - Zhu, Y. (2018). *Embedded Systems with ARM Cortex-M Microcontrollers in Assembly Language and C* (3rd ed.). E-Man Press LLC.
* [Pdf](../../docs/i2c-interface-ece362-engineering.purdue.edu.pdf "I2C interface")

