#![no_std]
#![no_main]


use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};


#[entry]
fn main() -> ! {
    rtt_init_print!();

    // Test a print.
    rprintln!("Hello, world!");

    // Test a panic.
    panic!("Panic");
}
