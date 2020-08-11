## Programming our chip

Now that we have built our project, we should upload it to our microcontroller, a process called "flashing". To do so, you need to install a tool called `cargo-flash`

```
cargo install cargo-flash
```

`cargo-flash` is a subcommand for cargo that acts like `cargo run` but for debug probes.

The command taks a `--chip` argument, corresponding to your processor. In the blue pill case, that should be `stm32f103c8`, and for the f3, it should be `STM32F303VCTx`. Using `cargo flash --list-chips` helps you to spot the identifier for other MCUs.

Running
```
cargo flash --chip <chip>
```

should then give you the following error:

```
Caused by:
    0: Error while flashing
    1: No flash memory contains the entire requested memory range 0xFFFFF000..0xFFFFF094.
```

This is because we, in an embedded system need to make sure that our code is placed in the correct place, so the processor can execute it. The fix is to add a file called `memory.x` specifying the address of RAM and FLASH in the processor. Copy the template file from the HAL repository and adapt it to your MCU. For [the f1 HAL](https://github.com/stm32-rs/stm32f1xx-hal/blob/master/memory.x) if you're using a blue pill, or [the f3 hal](https://github.com/stm32-rs/stm32f3xx-hal/blob/master/memory.x) if you're using the F3DISCOVERY.

Blue Pill's STM32F103C8 comes with 64 KiB Flash and 20 KiB RAM. This is a bit hard to find in its datasheet but could be easily seen at the [STM31F103 family overview](https://www.st.com/en/microcontrollers-microprocessors/stm32f103.html). The `memory.x` file from the [the f1 HAL](https://github.com/stm32-rs/stm32f1xx-hal/blob/master/memory.x) is exactly for this chip.

The STM32F3DISCOVERY's STM32F303VC comes with 256 KiB Flash and 48 KiB RAM (see [STM32F303 family overview](https://www.st.com/en/microcontrollers-microprocessors/stm32f303.html)) and again, the [memory.x from the HAL](https://github.com/stm32-rs/stm32f3xx-hal/blob/master/memory.x) is provided for exactly this chip.

You also need to tell the linker to use that file. Edit `.cargo/config` to add it to the configuration your your actual target. 

The following lines will do this for the Blue Pill board
```
[target.thumbv7m-none-eabi]
rustflags = [
  "-C", "link-arg=-Tlink.x",
]
```
and for the STM32F3DISCOVERY they look like:
```
[target.thumbv7em-none-eabihf]
rustflags = [
  "-C", "link-arg=-Tlink.x",
]
```

> If you're paying attention, you may wonder what we just did, we said this was to use the memory.x file, but we make no mention of it there.
> 
> What is actually happening is that the the Cortex-M runtime's linker script [`link.x`](https://github.com/rust-embedded/cortex-m-rt/blob/854aa2c7a15f23c6143037835d18c1e8c28903e4/link.x.in#L23) which we want to use for linking includes `memory.x` from the current project.

Cargo flash should now work correctly and look something like this:

```
  Erasing sectors   ✔ [00:00:00] [###############]  10.00KB/ 10.00KB @  17.30KB/s (eta 0s )
Programming pages   ✔ [00:00:00] [###############]  10.00KB/ 10.00KB @   4.44KB/s (eta 0s )
```

Unfortunately, our infinite loop is not very interesting to look at, but we'll get to fixing that soon.

Before we do though, you may get tired of writing `--chip <your chip>` every time you want to test your code. If you do, jump to the section on [cargo embed](#debug-messages-and-cargo-embed) to get a solution.
