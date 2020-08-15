#![no_std]


use log::{Log, Metadata, Record};
use rtt_target::rprintln;


pub struct RttTargetLogger;


// A simple logger implementation taken from
// https://github.com/ferrous-systems/embedded-trainings-2020/blob/7134ab798eb24e2c3240a9631fbdcb5fe5bf0585/boards/dk/src/lib.rs#L268.
impl Log for RttTargetLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::STATIC_MAX_LEVEL
    }


    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        rprintln!(
            "{}:{} -- {}",
            record.level(),
            record.target(),
            record.args()
        );
    }


    fn flush(&self) {}
}
