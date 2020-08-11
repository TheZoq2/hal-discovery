# Introduction

"Modern" embedded rust development is typically done using hardware abstraction layers (**HALs**) and **drivers** that talk to external hardware. They communicate using a project called [embedded-hal](https://crates.io/crates/embedded-hal). For more information about this relationship, see [this oxidize talk](...) **TODO**: Link my oxdize talk

This guide will show you how that workflow might look like. We will start off with flashing some firmware to the board. Then we'll do the "hello world" of the embedded world, blink an LED. Finally, we'll talk to some external hardware, in this case an accelerometer.

## Hardware

This tutorial is written with two hardware modules in mind, use whichever one you like best or can get your hands on.

If you have some other hardware in mind, you may still be able to follow along, though it will take a bit more effort. The requirements are that it has a HAL. [Other Hardware](#other-hardware) for details.

### blue-pill

The "blue-pill" is a cheap and small board based on the `stm32f103` processor.

It has no built in debugger/programmer so you need an external debug probe, like https://www.st.com/en/development-tools/stlink-v3mini.html (Though any probe compatible with [cargo flash](https://github.com/probe-rs/cargo-flash) should work)

*TODO*: Add an image

*TODO*: Make a note that there is no "one blue pill" and that slight differences can be present

### STM32F3DISCOVERY

The [STM32F3DISCOVERY](https://www.st.com/en/evaluation-tools/stm32f3discovery.html) is another common evaluation board with a STM32F3 series microcontroller. In contrast to the Blue Pill boar it comes with an on-board ST-LINK debugger which you could also configure for debugging external targets (like the Blue Pill board).

### Other hardware

If you don't want to use the above processors, you can still follow along. Make sure the processor you want to use has a HAL implementation, a list of which you can find at [awesome-embedded-rust](https://github.com/rust-embedded/awesome-embedded-rust#hal-implementation-crates)
