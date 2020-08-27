#![no_std]
#![no_main]


use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use lsm303dlhc::{AccelOdr, Lsm303dlhc, Sensitivity};
use micromath::F32Ext;
use micromath::vector::F32x3;
use nb::block;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;
use rtt_target_logger::RttTargetLogger;
use stm32f3xx_hal as hal;

use hal::gpio::gpioe::PEx;
use hal::gpio::{Output, PushPull};
use hal::i2c::I2c;
use hal::pac;
use hal::prelude::*;
use hal::time::{Hertz, KiloHertz};
use hal::timer::Timer;


const I2C_FREQUENCY: KiloHertz = KiloHertz(400);
const SAMPLE_FREQUENCY: Hertz = Hertz(10);

const ACC_G_PER_LSB: f32 = 1.0 / (1 << 14) as f32;
const COS_85_DEGREE: f32 = 0.087;
// Consider the board as flat if it is tilted less than +/- 5 degrees to make
// finding a flat position easy.
const G_UNIT_XY_FLAT_THRESHOLD: f32 = COS_85_DEGREE;


#[derive(Debug)]
enum Direction {
    North = 0,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}


trait F32x3Ext {
    fn euclidian_norm(v: &Self) -> f32;
    fn unit(v: &Self) -> Self;
}


impl F32x3Ext for F32x3 {
    fn euclidian_norm(v: &F32x3) -> f32 {
        (v.x * v.x + v.y * v.y + v.z * v.z).sqrt()
    }


    fn unit(v: &Self) -> Self {
        let n = Self::euclidian_norm(v);

        F32x3 {
            x: v.x / n,
            y: v.y / n,
            z: v.z / n,
        }
    }
}


fn g_from_raw_acc_value(g: &lsm303dlhc::I16x3) -> F32x3 {
    // Convert into the same reference frame as the gyroscope on the
    // STMF32DISCOVERY which we are about to use in later examples as well.
    F32x3 {
        x: g.y as f32 * ACC_G_PER_LSB,
        y: -g.x as f32 * ACC_G_PER_LSB,
        z: g.z as f32 * ACC_G_PER_LSB,
    }
}


fn board_is_flat(u: &F32x3) -> bool {
    u.x.abs() <= G_UNIT_XY_FLAT_THRESHOLD
        && u.y.abs() <= G_UNIT_XY_FLAT_THRESHOLD
}


fn tilt_direction(u: &F32x3) -> Option<Direction> {
    if board_is_flat(u) {
        None
    }
    else {
        // Let's use atan2_norm which returns a value from [0, 4) so there is
        // no need to fiddle around with pi.
        let at2n = u.x.atan2_norm(u.y);

        match at2n {
            // The first case contains the wrap-around of atan2_norms output
            // interval.
            _ if 3.75 < at2n || at2n <= 0.25 => Some(Direction::North),
            _ if 0.25 < at2n && at2n <= 0.75 => Some(Direction::NorthEast),
            _ if 0.75 < at2n && at2n <= 1.25 => Some(Direction::East),
            _ if 1.25 < at2n && at2n <= 1.75 => Some(Direction::SouthEast),
            _ if 1.75 < at2n && at2n <= 2.25 => Some(Direction::South),
            _ if 2.25 < at2n && at2n <= 2.75 => Some(Direction::SouthWest),
            _ if 2.75 < at2n && at2n <= 3.25 => Some(Direction::West),
            _ if 3.25 < at2n && at2n <= 3.75 => Some(Direction::NorthWest),
            _ => None,
        }
    }
}


#[entry]
fn main() -> ! {
    rtt_init_print!();

    log::set_logger(&RttTargetLogger).unwrap();
    log::set_max_level(log::STATIC_MAX_LEVEL);

    log::info!("startup");

    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);

    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), I2C_FREQUENCY, clocks,
        &mut rcc.apb1);

    let mut lsm303dlhc = Lsm303dlhc::new(i2c).unwrap();
    lsm303dlhc.accel_odr(AccelOdr::Hz400).unwrap();
    lsm303dlhc.set_accel_sensitivity(Sensitivity::G1).unwrap();


    let led_n = gpioe.pe9.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let led_ne = gpioe.pe10.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let led_e = gpioe.pe11.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let led_se = gpioe.pe12.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let led_s = gpioe.pe13.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let led_sw = gpioe.pe14.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let led_w = gpioe.pe15.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let led_nw = gpioe.pe8.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    // TODO: Is there a convenient way of handling all and individual LEDs at the same time?
    //
    // TODO: Is there somehting which we could use to enforce that the indices
    // below match the enum discriminants.
    let mut leds: [PEx<Output<PushPull>>; 8] = [
        led_n.downgrade(),
        led_ne.downgrade(),
        led_e.downgrade(),
        led_se.downgrade(),
        led_s.downgrade(),
        led_sw.downgrade(),
        led_w.downgrade(),
        led_nw.downgrade(),
    ];

    // TODO: Factor out to function.
    for led in leds.iter_mut() {
        led.set_low().unwrap();
    }



    let mut timer = Timer::tim2(dp.TIM2, SAMPLE_FREQUENCY, clocks,
        &mut rcc.apb1);

    log::info!("main loop");

    loop {
        block!(timer.wait()).unwrap();

        let g_raw = lsm303dlhc.accel().unwrap();
        let g = g_from_raw_acc_value(&g_raw);
        let g_unit = F32x3Ext::unit(&g);

        log::debug!("g_unit: ({}, {}, {})", g_unit.x, g_unit.y, g_unit.z);

        if board_is_flat(&g_unit) {
            log::debug!("board flat");

            // TODO: Factor out to function.
            for led in leds.iter_mut() {
                led.set_high().unwrap();
            }
        }
        else {
            let at2n = g_unit.x.atan2_norm(g_unit.y);
            let direction = tilt_direction(&g_unit);

            log::debug!("board tilted: at2n: {}, direction: {:?})", at2n, direction);

            // TODO: Factor out to function.
            for led in leds.iter_mut() {
                led.set_low().unwrap();
            }

            if let Some(direction) = direction {
                leds[direction as usize].set_high().unwrap();
            }
        }
    }
}
