#![no_std]
#![no_main]


use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use nb::block;
use panic_semihosting as _;
use stm32f3xx_hal as hal;

use hal::prelude::*;
use hal::stm32;
use hal::timer::Timer;


#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.sysclk(64.mhz()) .pclk1(32.mhz())
        .freeze(&mut flash.acr);

    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let mut led_pin = gpioe.pe9.into_push_pull_output(
        &mut gpioe.moder, &mut gpioe.otyper);

    let mut timer = Timer::tim6(dp.TIM6, 1.hz(), clocks, &mut rcc.apb1);

    loop {
        block!(timer.wait()).unwrap();
        led_pin.set_high().unwrap();

        block!(timer.wait()).unwrap();
        led_pin.set_low().unwrap();
    }
}
