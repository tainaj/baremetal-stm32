# Guide 2: How to setup Embedded Rust
The **Rust programming language** is a new addition to the programming suite, and it now supports embedded system development, called "Embedded Rust".

In [Vivinomicon's](https://vivonomicon.com/) post [Hello, Rust: Blinking LEDs in a New Language](https://vivonomicon.com/2019/05/23/hello-rust-blinking-leds-in-a-new-language/), most of the first step, toolchain setup, is left for the reader to complete. Which is saying something, because following the first few chapters of the provided Rust documentation is proving to be vital to understand what is really going on, and represents the wisdom of [Rustaceans](https://rustacean.net/) who went before me. This guide will be my attempt to pitch in.

The target board for this guide is a NUCLEO-L031K6 development board. For an application example, see my [bonus entry for Rust](../bonus/rust-blink/ "Rust Blink").

# Source material
Consult the following resources referenced by this guide.
* ["Embedded Rust" ebook](https://docs.rust-embedded.org/book/ "Embedded Rust ebook") - The first two chapters cover intallation and a step-by-step guide to setup your work environment to compile a Rust project.
* ["Discovery" ebook](https://docs.rust-embedded.org/discovery/ "Discovery ebook") - Written after Vivonomicon's "Rust Blink" post. Beginner-friendly examples of embedded Rust programs, which expand the concepts demonstrated here.
* ["Rust" ebook](https://doc.rust-lang.org/book/ "Rust ebook") - An introductory book about Rust, a programming language that gives you the option to control low-level details (such as memory usage) without all the hassle traditionally associated with such control.
* [Docs.rs](https://docs.rs/) - Look up entries for "svd2rust". Note that `stm32l0x1` here (a submodule of crate `stm32l0`) is the **crates.io** version; see **Key differences** below for a walkthrough between the differences between that module and the local library crate used in this project.

# Initial installation
For projects programmed with Embedded Rust, non-application files present in other posts, such as those from **boot_s** **vector_tables**, **device_headers**, and **ld** sections, will not be imported.

## Downloaded files
The only file download is ST Microelectronics' System View Description (SVD) file for the [target board](https://www.st.com/en/microcontrollers-microprocessors/stm32l0-series.html). Go to **CAD Resources**, under **System View Description**. You may get a zip file; download and extract it. The collection may target a wider selection of boards; locate the SVD file specific to you.

## Embedded Rust toolchain
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
`cargo-binutils` | LLVM tools used to inspect binaries | `cargo install` | <ul><li>Binary essentials: `cargo readobj`, `cargo size`, `cargo objdump`</li><li>See Chapter 2.1 for an example use</li></ul>
`cargo-generate` | Generate a Rust project template    | `cargo install` | <ul><li>`cortex-m-quickstart` - STM32 Cortex-M template</li><li>See Chapter 2.1 for an example use</li></ul>

**Recommendation:** Go to Chapters 1.2 and 1.3 of the Rust ebook, compile and run "Hello, World" and "Hello Cargo". Some notes about a few `cargo` project essentials:
* `cargo new` -> Commonly used to create Rust project templates, including a custom PAC. Use `cargo-generate` to generate templates for an Embedded Rust project instead.
* `cargo run` -> Compiles, then executes your project. If running an Embedded Rust project, be sure to run `openocd` on a separate terminal before invoking `cargo run`. Uses OpenOCD, with aid of helper files `openocd.cfg` and `openocd.gdb`. Be sure to run `openocd` first.
* `cargo clean` -> Removes autogenerated build files from the project directory. I'm surprised this wasn't mentioned anywhere obvious...

## Other essential packages
The following installations are mentioned in Chapter 1.4.1 of the Embedded Rust ebook:
Utility           | About                                                  | Installed by       | Features
------------------|--------------------------------------------------------|--------------------|------------------------------
`gdb-multiarch`   | Preferred by these ebooks          | `sudo apt install` | <ul><li>same toolset as `arm-none-eabi-gdb`</li></ul>
`openocd`         | "Open On-chip debugger"                                | `sudo apt install` | <ul><li>Alternative to **st-link's** `st-util` utility</li><li>Simplified debugging with helper files `openocd.cfg` and `openocd.gdb`</li><li>**Works for NUCLEO-L031K6 dev board!**</li></ul>
`qemu-system-arm` (optional) | Open-source ARM machine emulator            | `sudo apt install` | <ul><li>Write a program for various ARM devices, such as the `LM3S6965`, without any hardware!</li></ul>

Once all these packages are installed, you may practice by following examples from Chapter 2. Read the next section for my overview on key topics.

# Follow-up to Chapter 2 example projects
The following entries come from my understanding and musings of a few passages from the Rust ebooks while following the Chapter 2 programming examples; consult the ebooks for an official explanation.

## What are packages? Structs? How is all this code organized?
Reference: Chapter 7 of Rust ebook

* Crate. Also top-level module. Typically represent an entire STM32 family. Example:
  * `stm32l0` (STM32L0 MCU family)
* Module. May contain submodules, structs, and other stuff. Examples:
  * `stm32l0::stm32l0x1` (STM32L0x1 subfamily, which includes STM32L031K6)
  * `stm32l0:stm32l0x1::gpioa` (Module-form of GPIOA peripheral)
* Struct. Heterogeneous collection of other types, called *fields*. Contain methods that return data values, like primitives or Structs. Examples:
  * `let device_peripherals = stm32l0x1::Peripherals::take().unwrap();` (The struct `stm32l0x1::Peripherals` has a method chain `take().unwrap()` that transfers ownership of all peripheral modules from crate `stm32l0x1` to a new variable `device_peripherals`.)
  * `let ptr = stm32l0x1::GPIOB::ptr();` (The struct `stm32l0x1::GPIOB` has a method `ptr()` that licenses permission to borrow the GPIOB module to a new variable `ptr`. Remember to dereference `ptr` when calling for struct members!)

1. Methods are generally associated with each other through various implementation types, like the Struct itself.

## How openocd works in our work environment
Reference: Chapter 2.2 of Embedded Rust ebook

Let's understand the following command, shall we?
```
openocd -f interface/stlink-v2-1.cfg -f target/stm32l0.cfg
```
This command will initiate a connection with the ST-LINK interface for the STM32L0 device in TCP port 3333. Be sure to *open a new terminal* to do other tasks, as this command will *brick your current terminal*.

You may type the above command in its entirety, if you wish. But a shortcut would be to simply type:
```
openocd
```
This is made possible by the helper file `openocd.cfg`, which is used by openocd by default, if it exists. By the way, the locations for `interface/stlink-v2-1.cfg` and `target/stm32l0.cfg` are found in this directory:
```
/usr/share/openocd/scripts/
```
Again, by the way, `target/stm32l0.cfg` was an educated guess. `interface/stlink-v2-1.cfg`, on the other hand, required guess and check. Supposedly, the Rust ebook suggested `interface/stlink.cfg`, but I guess that was not available for me in the source directory.

## How gdb-multiarch works in our work environment
Reference: Chapter 2.2 of Embedded Rust ebook

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

# Author's Notes
Rust has some interesting features that make me curious. Consult the Rust ebook for a better explanation:
* Mutability. Variables are immutable by default. Making a variable mutable (with `mut`) announces that you expect that code elsewhere will change its value.
* Ownership. Except for the most primitive data types (`i32` or otherwise), this snippet `let s1 = ...; let s2 = s1;` causes `s1` to be moved into `s2`, meaning that `s2` claims ownership of `s1`'s content, making `s1` invalid.
* **Unsafe Rust.** For an advanced topic, you *must* encounter this early in order to implement meaningful features in Embedded Rust applications, such as interrupts.
* **Fearless concurrency.** Made possible by ownership and type systems. Essential knowledge to perfect embedded software that work with interrupts, real time operating systems, and *multiple cores*.
* Char type in Rust is based on Unicode (UTF-8), wihch is *4 bytes long*.
