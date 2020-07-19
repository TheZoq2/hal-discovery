#![no_std]
#![no_main]

extern crate panic_semihosting;

use cortex_m_rt::entry;

use stm32f1xx_hal as hal;
use hal::pac;

use hal::prelude::*;

use embedded_hal::digital::v2::OutputPin;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    let mut led_pin = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    led_pin.set_low();

    loop {
        continue;
    }
}
