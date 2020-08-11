## Project setup

As as usual, create your project with `cargo init`

Building an embedded project takes a bit more configuration than a normal rust project, because it needs to be cross compiled. While the process involves quite a few steps, you only have to do it once, so just bear with us :)


The first thing you have to do is add support for the architecture of the target processor which in our case is `thumbv7m-none-eabi` for Blue Pill's STM32F1 based on a Cortex-M3
```
rustup target add thumbv7m-none-eabi
```
and `thumbv7em-none-eabihf` where the suffix `hf` stands for hardware floating point support for the STM32F3DISCOVERY's STM32F3 based on a Cortex-M4
```
rustup target add thumbv7em-none-eabihf
```

You can now build your project for the Blue Pill board using 

```
cargo build --target thumbv7m-none-eabi
````

and for the f3discovery:

```
cargo build --target thumbv7em-none-eabihf
```

If you do, you'll get the following error: 

```
error[E0463]: can't find crate for `std`
  |
  = note: the `thumbv7m-none-eabi` target may not be installed

error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
```

Before we move on, you probably don't want to type `--target thumbv7m-none-eabi` every time you rebuild your project. To avoid that: create a file called `.cargo/config` with the following content for the Blue Pill board

```toml
[build]
target = "thumbv7m-none-eabi"
```

or 

```toml
[build]
target = "thumbv7em-none-eabihf"
```

for the STM32F3DISCOVERY board.


Rerun `cargo build` and verify that you now build for the thumb target by default. If you get the same error, you do!

Let's get back to making our code compile. The error occurs because
the standard library needs an operating system and there is no operating system on our target. The fix is fairly simple, build without the stdlib. To do so, add `#![no_std]` to the top of your `main.rs`. You also have to remove`println!` since that requires `std`

It should look like

```rust
#![no_std]
fn main() {
}
```

Rerun `cargo build` and you will get another error: 

```
error: `#[panic_handler]` function required, but not found

error: aborting due to previous error
```

This is because there is no default way to handle `panic` without an operating system. Fixing this error is a matter of adding an external crate that does panic handling. There are [several to chose from][panic handlers] depending on what you want.

- `panic-halt`: Stop execution and halt the program
- `panic-semihosting`: Report panic messages through the "semihosting" mechanism. Use this if you want error me- `panic-semihosting`: Report panic messages through the "semihosting" mechanism. Use this if you want error messages in your debugger log

- And a whole lot more, see the linked [crates.io][panic handlers] keyword

[panic handlers]:https://crates.io/keywords/panic-handler

Pick one you like, we'll use `panic-rtt` since we have a debugger and seing panic messages when they happen is very convenient.

Add the crate to your `Cargo.toml`

```toml
panic-rtt = "0.3.0"
```

And then add a use statement for it. Unfortunately, rustc will not realise that it is used and throw us a warning, which is the reason for the `as _` part.

```rust
use panic_rtt as _;
```

If we build it now, we get... another error:

```rust
error: requires `start` lang_item

error: aborting due to previous error
```

This one says that the compiler doesn't know where our program should start. (even though we have a main function). This fix takes a few lines of code.

First: add the `#![no_main]` attribute. Then, add the crate `cortex-m-rt` which provides, among other things, an attribute that marks a function as the entry point of the program.

Finally, modify your code like this:

```rust
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_rtt as _;

#[entry]
fn main() -> ! {
    loop {
        continue;
    }
}
```

This marks `main` as the entry point. The function signature is changed to return `!`, the "never type". This is because your program should never exit, there is no OS to take over execution. Instead, the microcontroller will just blindly execute the instructions after the main function.

We can't just say that `main` should never return, we also need to make it so. Hence the `loop {}` Finally, the `continue` inside the loop is there to fix an LLVM bug where empty loops can be optimised away.

`cargo build` should now work as expected!
