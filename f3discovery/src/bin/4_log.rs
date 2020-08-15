#![no_std]
#![no_main]


use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;
use rtt_target_logger::RttTargetLogger;


fn init() {
    rtt_init_print!();

    log::set_logger(&RttTargetLogger).unwrap();
    // Log everything which got compiled into our binary. STATIC_MAX_LEVEL gets
    // defined depending on the feature configuration for the log crate in
    // Cargo.toml (see https://docs.rs/log/0.4.11/log/#compile-time-filters for
    // details).
    log::set_max_level(log::STATIC_MAX_LEVEL);
}


#[entry]
fn main() -> ! {
    init();

    // Test log output. With the maximum compile-time log level info set in
    // Cargo.toml  only the first message will actually end up in the binary
    // and get logged.
    log::info!("Hello logging!");
    log::debug!("Some verbose debug output.");

    // Test a panic.
    panic!("Panic");
}
