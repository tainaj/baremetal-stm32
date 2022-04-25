//! Using a device crate
//!
//! Crates generated using [`svd2rust`] are referred to as device crates. These crates provide an
//! API to access the peripherals of a device.
//!
//! [`svd2rust`]: https://crates.io/crates/svd2rust
//!
//! This example depends on the [`stm32f3`] crate so you'll have to
//! uncomment it in your Cargo.toml.
//!
//! [`stm32f3`]: https://crates.io/crates/stm32f3
//!
//! ```
//! $ edit Cargo.toml && tail $_
//! [dependencies.stm32f3]
//! features = ["stm32f303", "rt"]
//! version = "0.7.1"
//! ```
//!
//! You also need to set the build target to thumbv7em-none-eabihf,
//! typically by editing `.cargo/config` and uncommenting the relevant target line.
//!
//! ---

#![no_main]
#![no_std]

#[allow(unused_extern_crates)]
use panic_halt as _;

//use cortex_m::peripheral::syst::SystClkSource; // use later...
use cortex_m_rt::entry;
//use cortex_m_semihosting::hprint; // use later...
//use stm32l0::stm32l0x1::{interrupt, Interrupt, NVIC};
//use stm32l0::stm32l0x1::{self, interrupt};
use stm32l0x1::{self, interrupt};

#[entry]
fn main() -> ! {
    // Acquire the device peripherals. They can only be taken once ever.
    //let device_peripherals = stm32l0x1::Peripherals::take().unwrap();

    // Get a reference to GPIOA and RCC to save typing.
    //let gpioa = &device_peripherals.GPIOA;
    //let rcc = &device_peripherals.RCC;
    //let tim2 = &device_peripherals.TIM2;

    // Enable the GPIOA clock ans set PA8 (good) to be an output
    //rcc.ahbenr.modify(|_, w| w.gpioaen().enabled());


    //let p = cortex_m::Peripherals::take().unwrap();

    //let mut syst = p.SYST;
    //let mut nvic = p.NVIC;

    //nvic::unmask(Interrupt::EXTI0_1);

    // configure the system timer to wrap around every second
    /*syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000); // 1s
    syst.enable_counter();*/

    loop {
        // busy wait until the timer wraps around
        //while !syst.has_wrapped() {}

        // trigger the `EXTI0` interrupt
        //NVIC::pend(Interrupt::EXTI0);
    }
}

/*#[interrupt]
fn EXTI0() {
    hprint!(".").unwrap();
}*/
