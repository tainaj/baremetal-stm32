# Part 1: Hello, ARM! & Part 2: Making it to 'Main'
This repository is a submit box for Part 1 and Part 2 of Vivonomicon's **"Bare Metal" STM32 Programming** blog series:

https://vivonomicon.com/2018/04/02/bare-metal-stm32-programming-part-1-hello-arm/
https://vivonomicon.com/2018/04/20/bare-metal-stm32-programming-part-2-making-it-to-main/

For future reference, differences between my code and Vivonomicon's will be highlighted in the Procedure section.

## Hardware
For this lab, I used both the STM32F0 Discovery board; other boards are mentioned in the blog:

Development board | STM32F0DISCOVERY | NUCLEO-F031K6
------------------|------------------|---------------
Microcontroller   | STM32F051R8T6    | STM32F031K6
Core              | Cortex-M0        | Cortex-M0
Flash memory      | 64 KB            | 32 KB
SRAM              | 8 KB             | 4 KB
Max frequency     | 48 MHz           | 48 MHz
Package           | LQFP64           | LQFP32

## Software
The following software installed in this part have these notes:
### GNU Arm Embedded Toolchain
Since Ubuntu 14.04, ARM aren't distributing their tools via PPA (via `sudo apt-get install`) anymore in the form of `gcc-arm-embedded`, a package that includes both `gcc-arm-none-eabi` and `gdb-arm-none-eabi` tools used by Vivonomicon. Currently, any existing packages found via PPA are outdated.

See the following Stack Exchange link to learn more about installing the latest version without PPA:

https://askubuntu.com/questions/1243252/how-to-install-arm-none-eabi-gdb-on-ubuntu-20-04-lts-focal-fossa/1243405#1243405

Remember to create additional links for binaries `arm-none-eabi-objcopy` and `arm-none-eabi-objdump`.

### ST-Link
Link to Texane's (now stlink-org) open-source STLink tool here:

https://github.com/stlink-org/stlink

To install, follow the subheaders for Linux compiling, as described in their document `doc/compling.md`:
* Common requirements
* Installation
* Building
* Set device access permissions and the role of udev

https://github.com/stlink-org/stlink

Note 1: Some commands, like `make install`, requires the use of `sudo` to complete and will fail otherwise.

Note 2: If you get the error **"cannot open shared object file: No such file or directory"**, try `sudo /sbin/idconfig -v`

### vector_table.S, core.S
For this lab, I copied the files written for the STM32F031K6 since hardware interrupts don't matter right now. For reference, you may find the content for these files that corresponds to your chip in its associated startup file provided by STM32Cube:

https://github.com/STMicroelectronics/STM32CubeF0/tree/master/Drivers/CMSIS/Device/ST/STM32F0xx/Source/Templates/gcc

## Procedure (Part 1)
### The toolchain, linker script, vector table, 'Hello World', compiling the code
Install the following packages from your package repository:
* `arm-none-eabi-gcc`
* `gdb-multiarch`

In a dedicated project folder, create the linker script `STM32F051R8T6.ld` as written in the example. For `_estack` and `MEMORY`, insert the values that correspond for the F051R8 device used in this lab.

Create `core.S` as written in the example. Run `gcc-arm-none-eabi` to compile `main.elf` as described in Part 1.

### Uploading, running, and debugging
Follow the steps in the link under **ST-Link** to download their open-source tooling to a folder of your choosing.

Run `st-util`. It will run continuously unless you press CTRL-C.

In a new terminal window, go to your project folder and start the debugger as described in Part 1, substituting `gdb-multiarch` for `gdb-arm-none-eabi`. Compare results.

## Procedure (Part 2)
Do the following additions to your project folder, as descibed in Part 2:

* Update `STM32F051R8T6.ld` wich a new SECTIONS block.
* Create `vector_table.S`. To adequately match your Discovery board, find the corresponding startup file to use as a guide.
* Update `core.S` with boot logic.
* Create `main.c`.
* Create `Makefile` to simplify the build process.

Compile a new `main.elf` file, and run the debugger, as described in Part 2. Compare results. 

## Author's notes 
For this part, folder content is the final result after Part 2.
