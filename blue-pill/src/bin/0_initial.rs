#![no_std]
#![no_main]

use panic_rtt as _;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    loop {
        continue;
    }
}
