#![no_std]
#![no_main]


use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::rtt_init_default;


#[entry]
fn main() -> ! {
    rtt_init_default!();

    loop {
        // Don't let this loop get optimized away.
        continue;
    }
}
