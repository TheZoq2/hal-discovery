## Turning on an LED

Now that we *finally* have a way to build and run our project we can do something useful! Let's start with the "hello world" of embedded, turning on an LED.

In rust, the nicest way to program microcontrollers is to use a Hardware Abstraction Layer (HAL). For the stm32f1 chip on the blue pill, you should use [stm32f1xx_hal](https://crates.io/crates/stm32f1xx-hal), and for the f3, you should use [stm32f3xx_hal](https://crates.io/crates/stm32f1xx-hal).

Add the relevant HAL to your `Cargo.toml`. the `xx` part of the HAL names indicates that they support several similar microcontroller. This means that we have to chose which one we want in particular. For the blue pill, that would be `stm32f103`, and for the f3discovery, `stm32f303xc`.

For the blue pill, add the following to your `Cargo.toml`
```toml
[dependencies.stm32f1xx-hal]
features = ["stm32f103"]
version = "0.6.1"
```
and likewise the fragment
```toml
[dependencies.stm32f3xx-hal]
features = ["stm32f303xc"]
version = "0.5.0"
```
for the STM32F3DISCOVERY board.

For convenience, we'll import the HAL using an alias. We'll also include the prelude defintions which contain a few extension traits that we will need down the line.

```rust
use stm32f1xx_hal as hal; // If blue pill
use stm32f3xx_hal as hal; // If f3discovery

use hal::prelude::*;
```

The first step in using a HAL is to get access to the `peripherals` of your microcontroller. These are structs that wrap the internal registers. They are bundled in a crate called Peripheral Access Crate (PAC) which is re-exported by the hals as `pac`.

Let's get ourselves a peripheral struct:

```rust
use hal::pac;

fn main() -> ! {
    // dp is the "device peripherals".
    let dp = pac::Peripherals::take().unwrap();
    
    // loop { ...
}
```


The reason we have to do this, is that it ensures that we only have *one* instance of `dp`. Call `take` more than once, and you will get an error.

This in turn, allows the ownersip system to guarantee that you don't accidentally re-use the same peripheral multiple times. This will probably become clearer later

The HALs fullfill a similar task as arduino: providing a high level interface around all the peripherals in the microcontroller. However, unlike Arduino, only the input and output code has a shared API, intial setup is HAL dependent.

A lot of the details about how to use these HALs can be found in the documentation. We'll link to the relevant pieces as we go along. Try to follow along with how we navigate the docs yourself, it will greatly help you when working on your own projects :)

### Configuring the blue pill

Our goal is to turn on an LED, that is typically done using *GPIO* pins. Both the f1 and f3 HALs have a GPIO module, so let's start there ([F1 GPIO module](https://docs.rs/stm32f1xx-hal/0.6.1/stm32f1xx_hal/gpio/index.html), [F3 GPIO Module](https://docs.rs/stm32f3xx-hal/0.4.3/stm32f3xx_hal/gpio/index.html)). The LED on the blue pill is connected to `PC13`, which as the name implies is in GPIO bank C. The F3DISCOVERY has a bunch of LEDs, but we'll just use one of them, `PE9`.

The docs mention that you first need to aquire and configure the GPIO peripheral using

```rust
let mut gpioa = dp.GPIOC.split(&mut rcc.apb2);
```

The `split` function comes from the trait `GpioExt` in both cases, and reading the docs for that ([f1][f1 GpioExt], [f3][f3 GpioExt]) shows that they need a mutable reference to [AHB][f3 AHB] and [APB2][f1 APB2], respectively.

[f1 GpioExt]: https://docs.rs/stm32f1xx-hal/0.6.1/stm32f1xx_hal/gpio/trait.GpioExt.html#tymethod.split
[f3 GpioExt]: https://docs.rs/stm32f3xx-hal/0.4.3/stm32f3xx_hal/gpio/trait.GpioExt.html#tymethod.split
[f3 AHB]: https://docs.rs/stm32f3xx-hal/0.4.3/stm32f3xx_hal/rcc/struct.AHB.html
[f1 APB2]: https://docs.rs/stm32f1xx-hal/0.6.1/stm32f1xx_hal/rcc/struct.APB2.html

These structs are both members of a peripheral in the processor that handles reset and control of other peripherals, `RCC`. If you navigate to those structs, the documentation in both crates ([f1][f1 APB2], [f3][f3 AHB]) should tell you how to aquire the structs

*(Sidenote: as the author of the f1 hal, it is my goal that the docs should always work like this. See a struct, click or search for it and find how to get your hands on one. If that's not the case, please, open an issue :))*

**Blue pill**
```rust
let mut rcc = dp.RCC.constrain();
let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);
```

**F3DISCOVERY**
```rust
let mut rcc = dp.RCC.constrain();
let mut gpioe = dp.GPIOC.split(&mut rcc.ahb);
```

**NOTE** I would like to put some sort of info box with the next section. This is *probably* possible with gitbook, but not with pure markdown. For now, I'll use quotes, which is ugly
#### What happened here?

> This code might be pretty confusing, there are lots of strange acronyms being thrown around the code. If you just want to accept this at face value, and it is the goal of the HAL that this should be possible, skip this section.
>
> To understand why this is done, you need some information about how the stm32f1 processor works. It has a central peripheral called `rcc` that *resets* and *controls* every other peripheral (GPIO, SPI, USB etc.). To use any other peripheral, it first needs to be turned on in this RCC peripheral.
>
> The constrain function turns on, and configures the `rcc` peripheral to put it in a known state. Then, the split function on `gpioc` turns on the GPIO bank, and creates a struct of individual pins. Thanks to the rust type system, it is impossible to forget to turn on the GPIO bank in rcc, if you do, you can not call `split`. This is a common feature across the embedded ecosystem and a key selling point.


Once the GPIO struct is configured, we can access all the pins in the bank using `gpioc.pcX`. To turn on the LED, we need to do two things: put the pin in an output mode, and setting the output.

The functions for doing so can be found in the documentation for the individual pins ([f1][f1_pin_doc], [f3][f3_pin_doc]): 

The pins have several functions on the form `into_x` where `x` is different modes. For driving an LED, we want the push_pull mode. That gives us a `Pxx<Output<PushPull>>` struct, which as you may be able to tell by the docs implements the trait `OutputPin`.

This trait contains 2 functinos: `set_high` and `set_low` which is what set the actual output value.

Thus, the code to configure a pin as output and turning the led on looks like this:

**Blue pill**

The LED on the board gets driven by sinking current to the GPIO pin. This might seem counterintuitive in the first place but usually MCUs are able to sink more current than they could source. So the pin has to be set low for turning on the LED:
```rust
let mut led_pin = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
led_pin.set_low().unwrap();

// While it may seem counterintuitive to set the value to low when we want it on, that is how the LED is wired up on most blue pills.
```

**F3DISCOVERY**

The LD3 on the board is sourced with current from the GPIO pin. So we have to set the pin high to turn on the LED:
```rust
let mut led_pin = gpioe.pe9.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
led_pin.set_high().unwrap();
```

The registers passed to the into_functions are registers that are used to set the mode, without them, no change can be made. By passing them as mutable references, we ensure that noone is using them elsewhere which might lead to race conditions. Which registers to pass can be seen in the documentation. 

[f1_pin_doc]: https://docs.rs/stm32f1xx-hal/0.6.1/stm32f1xx_hal/gpio/gpioc/struct.PC13.html
[f3_pin_doc]: https://docs.rs/stm32f3xx-hal/0.4.3/stm32f3xx_hal/gpio/gpioe/struct.PE9.html

If you compile this, you will get an error like this:

```
error[E0599]: no method named `set_low` found for struct `stm32f1xx_hal::gpio::gpioc::PC13<stm32f1xx_hal::gpio::Output<stm32f1xx_hal::gpio::PushPull>>` in the current scope
  --> src/1_led.rs:21:13
   |
21 |     led_pin.set_low().unwrap();
   |             ^^^^^^^ method not found in `stm32f1xx_hal::gpio::gpioc::PC13<stm32f1xx_hal::gpio::Output<stm32f1xx_hal::gpio::PushPull>>`
   |
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
6  | use embedded_hal::digital::v2::OutputPin;
```

This is because the `set_high` and `set_low` functions are from traits defined in [embedded-hal](https://crates.io/crates/embedded-hal). We'll get to what this crate does and why it is useful soon, but for now, add it as a dependency and follow the advice of the compiler, adding the use statement to your main file

```toml
embedded-hal = "0.2.4"
```

```rust
use embedded_hal::digital::v2::OutputPin;
```

Build and flash your project and your LED should light up!

Full example code can be found in [blue-pill/1_led.rs] and [f3discovery/1_led.rs]

[blue-pill/1_led.rs]: https://github.com/TheZoq2/hal-discovery/blob/master/blue-pill/src/bin/1_led.rs
[f3discovery/1_led.rs]: https://github.com/TheZoq2/hal-discovery/blob/master/f3discovery/src/bin/1_led.rs

> As you may have already noticed, digital I/O pin traits are provided by the module `digital::v2`. This is an updated version of this trais which adds a return value to indicate errors (which could happen for example when using an I/O expander over I2C). See https://therealprof.github.io/blog/digital-v1-to-digital-v2/ for more details.
