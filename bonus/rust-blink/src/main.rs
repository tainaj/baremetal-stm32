#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

// includes
//use cortex_m_semihosting::hprint; // uncomment this and the call to demonstrate debugging print statements (gdb only)
use cortex_m_rt::entry;
use stm32l0x1::{self, interrupt};

#[entry]
fn main() -> ! {
    // Acquire the device peripherals. They can only be taken once ever.
    let device_peripherals = stm32l0x1::Peripherals::take().unwrap();

    // Get a reference to GPIOB and RCC to save typing.
    //let gpiob = device_peripherals.GPIOB; // who is owner of what? :|
    //let mut gpiob = device_peripherals.GPIOB; // 'mut' not needed
    let gpiob = &device_peripherals.GPIOB; // assures 'device_peripherals' is still owner
    let rcc = &device_peripherals.RCC;
    let tim2 = &device_peripherals.TIM2;

    // Enable the GPIOB clock.
    //rcc.iopenr.modify(|_, w| w.iopben().set_bit()); // does not overwrite other register fields (recommended)
    //rcc.iopenr.modify(|r, w| w.iopben().set_bit()); // same as above, "r" is unused here
    //rcc.iopenr.modify(|_, w| w.iopben().enabled() ); // enabled() only valid with crates.io version
    rcc.iopenr.write(|w| w.iopben().set_bit()); // rewrite register from reset (okay here, but be careful)

    // Set PB3 to be a push-pull output.
    //gpiob.moder.modify(|_, w| w.mode3().bits(0x01)); // safe only with crates.io version
    //unsafe { gpiob.moder.modify(|_, w| w.mode3().bits( 0x01 ) ); } // equivalent to below
    gpiob.moder.modify(|_, w| unsafe { w.mode3().bits(0x01) }); // valid
    gpiob.otyper.modify(|_, w| w.ot3().clear_bit());

    // Set up the timer for slow interrupt generation
    // NOTE(unsafe): The psc field has not been sufficently documented
    // to allow safe writing of arbitrary integer values, so we have to
    // use unsafe here. This could be fixed by improving the SVD file.
    rcc.apb1enr.modify(|_, w| w.tim2en().set_bit());
    tim2.dier.write(|w| w.uie().set_bit());
    tim2.psc.write(|w| unsafe { w.psc().bits(1000) }); // safe only with crates.io version
    //tim2.arr.write(|w| w.arr().bits(2000)); // local crate defines arr_l() and arr_h() for TIM2
    tim2.arr.write(|w| unsafe { w.arr_l().bits(2000) }); // safe only with crates.io version ( with arr() )
    tim2.cr1.write(|w| w.cen().set_bit());

    // Enable the timer interrupt in the NVIC.
    unsafe { cortex_m::peripheral::NVIC::unmask(stm32l0x1::Interrupt::TIM2) };

    // The main thread can now go to sleep.
    // WFI (wait for interrupt) puts the core in sleep until an interrupt occurs.
    loop {
        cortex_m::asm::wfi();
    }
}

// Interrupt handler for TIM2
#[interrupt]
fn TIM2() {
    // NOTE(unsafe): We have to use unsafe to access the peripheral
    // registers in this interrupt handler because we have already used `take()`
    // in the main code. In this case all our uses are safe, not least because
    // the main thread only calls `wfi()` after enabling the interrupt, so
    // no race conditions or other unsafe behaviour is possible.
    // For ways to avoid using unsafe here, consult the Concurrency chapter:
    // https://rust-embedded.github.io/book/concurrency/concurrency.html

    // Clear the UIF bit tio indicate the interrupt has been serviced
    unsafe { (*stm32l0x1::TIM2::ptr()).sr.modify(|_, w| w.uif().clear_bit()) };

    // Read ODR3 to see if the pin is set, and if so, clear it,
    // otherwise, set it. We use the atomic BSRR register to
    // set/reset it without needing to read-modify-write ODR.
    let ptr = stm32l0x1::GPIOB::ptr();
    unsafe {
        if (*ptr).odr.read().od3().bit() {
            (*ptr).bsrr.write(|w| w.br3().set_bit());
        } else {
            (*ptr).bsrr.write(|w| w.bs3().set_bit());
        }
    }

    //hprint!(".").unwrap(); // see the include for more...
}