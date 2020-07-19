#![no_std]
#![no_main]


use cortex_m_rt::entry;
use panic_semihosting as _;
use stm32f3xx_hal as hal;

use hal::stm32;


#[entry]
fn main() -> ! {
    // TODO: How to get the peripherals set up correctly?
    // let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    // let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
    // let mut led = gpioe.pe3.into_push_pull_output(
    //     &mut gpioe.moder, &mut gpioe.otyper);

    // led.set_high().unwrap();

    loop {
        // Don't let this loop get optimized away.
        continue;
    }
}
