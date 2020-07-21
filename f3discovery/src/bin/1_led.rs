#![no_std]
#![no_main]


use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use panic_semihosting as _;
use stm32f3xx_hal as hal;

use hal::pac;
use hal::prelude::*;



#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);

    let mut led_pin = gpioe.pe9.into_push_pull_output(
        &mut gpioe.moder, &mut gpioe.otyper);

    led_pin.set_high().unwrap();

    loop {
        // Don't let this loop get optimized away.
        continue;
    }
}
