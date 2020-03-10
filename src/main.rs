#![no_main]
#![no_std]

// Halt on panic
extern crate embedded_hal;
#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt; // panic handler

// use core::fmt::Write;
// use core::fmt::Write;
use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

// use embedded_hal::fmt;
use hal::{nb::block, prelude::*, stm32};

use gb_rw_firm::gpio::{GpioPortA, GpioPortB, GpioPortC};
use gb_rw_firm::serial::Serial;
use gb_rw_firm::serial::Write as _;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    // Set up the system clock. We want to run at 48MHz for this one.
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(96.mhz()).freeze();

    // USART2
    // Configure pa2 as a push_pull output, this will be the tx pin
    // let tx = gpioa.pa2.into_alternate_af7();
    // // Take ownership over pa3
    // let rx = gpioa.pa3.into_alternate_af7();

    // Set up the usart device. Taks ownership over the USART register and tx/rx pins. The rest of
    // the registers are used to enable and configure the device.

    // let (tx, rx) = hal::serial::Serial::usart2(
    //     dp.USART2,
    //     (tx, rx),
    //     hal::serial::config::Config::default().baudrate(921600.bps()),
    //     clocks,
    // )
    // .unwrap()
    // .split();

    // let mut serial = Serial::new(rx, tx);

    // serial.write_all(b"\nHELLO\n").ok();

    let gpioa = GpioPortA::take(dp.GPIOA.split(), 1 << 5);
    let gpioa = gpioa.into_push_pull_output();
    // gpioa.write(0xffff).ok();

    // Create a delay abstraction based on SysTick
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    use stm32::GPIOA;
    use stm32f4::stm32f411 as stm32;

    // Set up the LED. On the Nucleo-411RE it's connected to pin PA5.
    // let gpioa = dp.GPIOA.split();
    // let mut led = gpioa.pa5.into_push_pull_output();

    let i = 5;
    // let offset = 2 * i;

    // unsafe {
    //     &(*GPIOA::ptr())
    //         .pupdr
    //         .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b00 << offset)));
    //     &(*GPIOA::ptr())
    //         .otyper
    //         .modify(|r, w| w.bits(r.bits() & !(0b1 << i)));
    //     &(*GPIOA::ptr())
    //         .moder
    //         .modify(|r, w| w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset)))
    // };

    // On for 1s, off for 1s.
    // led.set_high().unwrap();
    gpioa.write(0xffff).ok();
    // unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << (5 + 16))) };
    // unsafe { (*GPIOA::ptr()).bsrr.write(|w| w.bits(1 << i)) };
    delay.delay_ms(500_u32);
    // led.set_low().unwrap();
    gpioa.write(0x0000).ok();
    // unsafe { (*GPIOA::ptr()).bsrr.write(|w| w.bits(1 << (i + 16))) };
    delay.delay_ms(500_u32);

    let gpioa = gpioa.release();
    // Set up the LED. On the Nucleo-411RE it's connected to pin PA5.
    let mut led = gpioa.pa5.into_push_pull_output();

    loop {
        // On for 1s, off for 1s.
        led.set_high().unwrap();
        delay.delay_ms(500_u32);
        led.set_low().unwrap();
        delay.delay_ms(500_u32);
    }
}