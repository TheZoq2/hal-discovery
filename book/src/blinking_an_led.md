### A blinking LED

What's more interesting than a static LED, a blinking one of course!

To blink an LED, we need some way to keep track of time and delay before toggling the LED. Both our HALs have abstractions around timers ([f1 timer module], [f3 timer module])

[f1 timer module]: https://docs.rs/stm32f1xx-hal/0.6.1/stm32f1xx_hal/timer/index.html
[f3 timer module]: https://docs.rs/stm32f3xx-hal/0.4.3/stm32f3xx_hal/timer/index.html

If you open those links you may see that, at least at the time of writing this document, the documentation is quite barebones. This is unfotunately quite common in HALs, so while this will hopefully get fixed eventually, for now, working with HALs in rust requires some reading between the lines. We'll do our best to explain how to do this here :). Again, try to follow along in the docs while reading this section

We already found the timer module, let's look at its contents. Typically, HALs will have a module for each periheral kind, with a "main " struct for that peripheral, typically called the same thing as the module.

Sure enough, both the [f1][f1 timer struct] and [f3][f3 timer struct] have a struct in the module called `Timer`.

[f1 timer struct]: https://docs.rs/stm32f1xx-hal/0.6.1/stm32f1xx_hal/timer/struct.Timer.html
[f3 timer struct]: https://docs.rs/stm32f3xx-hal/0.4.3/stm32f3xx_hal/timer/struct.Timer.html

Looking at the docs, the methods on the Timer struct differ quite a bit because the f1 has a more fleshed out HAL in that regard (containing implementations for PWM, QEI etc.).

Since the docs differ a bit, we'll go through them both one at a time, stating with the [f1][f1 timer struct]. Even if you work with the f3, we advise you follow along with this section, we'll show some general lessons that you can take with you.

Since we want to use the timer as a plain old timer, we should look for that in the docs. There are a few methods called `start_count_down` that give us a [CountDownTimer][f1 CountDownTimer]. Looking at the implementation of that, we find functions like `reset`, `stop`, and if we scroll down to the trait implementations, `start` and `wait` as part of `embedded_hal`.

[f1 CountDownTimer]: https://docs.rs/stm32f1xx-hal/0.6.1/stm32f1xx_hal/timer/struct.CountDownTimer.html

From this, we can assume that this is what we want. The fact that the important methods, `wait` and `start` are in `embedded-hal` is also a bit of a clue, as that is where most important functions often reside.

Let's go back to one of the [start_count_down][f1 start_count_down] functions. as you can see, it consumes a `Self`. Looking just above it, we find a function called [tim2][f1 tim2] that creates `Self`, perfect.

[f1 start_count_down]: https://docs.rs/stm32f1xx-hal/0.6.1/stm32f1xx_hal/timer/struct.Timer.html#method.start_count_down
[f1 tim2]: https://docs.rs/stm32f1xx-hal/0.6.1/stm32f1xx_hal/timer/struct.Timer.html#method.tim2

The first argument to that function is a `pac::TIM2`. Just like the `pac::GPIOx` struct we used earlier, almost all `pac::SOMETHING` structs come from the device peripherals `dp` that we alrerady have access to.

The second argument is `Clocks`, clicking that link, we find some information about it, crucially a guide on how to construct it:

```rust
let mut flash = dp.FLASH.constrain();
let clocks = rcc.cfgr.freeze(&mut flash.acr);
```

The final argument, `APB1` also has docs about how to aquire an instance. It's just a member of `rcc` that we already have access to.

Thus, we now have everything we need to instanciate our timer, the code looks like this

```rust
// Same init code as before
let mut flash = dp.FLASH.constrain();
let clocks = rcc.cfgr.freeze(&mut flash.acr);
let timer = Timer::tim2(dp.TIM2, clocks, &mut rcc.apb1);
```

Now, all we have to do is create our CountDownTimer using [start_count_down][f1 start_count_down]. The only argument to that is a value of type `Into<Hertz>`. Clicking on that shows that all we have to do to get one of those is to call the `.hz()` method on an integer. Let's use that

```rust
let timer = timer.start_count_down(1.hz());
```

Wich will make the timer count down from 1 second.

Thus, all that remains is to use the [embedded-hal wait method] which returns a `Result<(), nb::Error<Void>>`. The (nonblocking) [nb crate] provides some utilites for working with async like things. The idea behind `nb` is for functions that wait for things that will happen eventually to return the above Result type. The `nb::Error<E>` type is a compound type like

```rust
pub enum Error<E> {
    Error(E),
    WouldBlock
}
```

When the returned value is `WouldBlock`, the operation is not done yet, otherwise the result is either `Ok` or `Error(E)`. E in our case is `Void`, indicating that it is not possible for an actual error to occur.

<!-- UPDATE NOTE: Void will probably be replaced at some point-->

So, to wait for our timer to reach zero, we would do something like
```rust
while let Err(nb::Error::WouldBlock) {}
// We got past, so the timeout is reached
```

Conveniently, the `nb` crate contains a macro for just that called [block!](https://docs.rs/nb/1.0.0/nb/macro.block.html).

[embedded-hal wait method]: https://docs.rs/stm32f1xx-hal/0.6.1/stm32f1xx_hal/prelude/trait._embedded_hal_timer_CountDown.html#tymethod.wait
[nb crate]: https://crates.io/crates/nb

Let's put all of this together. Add `nb` as a dependency:

```toml
# NOTE: at the time of writing, nb 1.0 is released, but the HALs still use `0.1.3`
nb = "0.1.3"
```

Using different versions of a crate due to transitive dependencies might sometimes result in strange issues, so have a look at your transitive depencencies (for example by `cargo tree`) and adjust your top-level dependencies in case of doubt.

And we'll replace the loop we had earlier with
```rust
loop {
    // NOTE: This unwrap is safe, timer.wait() returns Void
    block!(timer.wait()).unwrap();
    led_pin.set_low().unwrap();
    block!(timer.wait()).unwrap();
    led_pin.set_high().unwrap();
}
```


**TODO** Link to the example code once we're done 
<!-- UPDATE NOTE: Change when we update to `nb` 1.0 -->

Alright, now that we have a blinking LED on the F1, let's do the same thing with the f3.

Again, we'll start at the documentation of the [f3 timer module]. It is significantly simpler than the F1, only containing a `Timer` struct and some helper things. Looking at the [Timer struct][f3 timer struct], we find things which are similar to the `CountDownTimer` of the f1 crate. Scroll down and you'll find implementations of the [embedded-hal CountDown trait], that contain the `wait` and `start` methods we're looking for.

So, let's instanciate ourselves a `Timer`. If you look at the docs, you'll again find a bunch of functions with the name `timX` that produce `Self`. Sounds like what we're looking for

Looking at the [type signature][f3 tim2], we see that it's quite similar to the f3, it takes `pac::TIM2`, `clocks` and `APB1`. Unfortunately, those registers don't directly show how to acquire those structs. One thing we can do here is to look for some examples. There is no example for using the timers, but the [spi example](f3 spi example) shows you how to acquire a `clocks` struct, and we can do a bit of guesswork, that the `APB1` struct is part of rcc, since the example shows `rcc.apb2`.

Putting it all together, we get the following: 
```rust
let mut flash = dp.FLASH.constrain();
let clocks = rcc.cfgr.freeze(&mut flash.acr);

let mut timer = Timer::tim2(dp.TIM2, 1.hz(), clocks, &mut rcc.apb1);

loop {
    block!(timer.wait()).unwrap();
    led_pin.set_high().unwrap();

    block!(timer.wait()).unwrap();
    led_pin.set_low().unwrap();
}
```
The examples differ only in initializing and starting the timer. As this is not part of the embedded HAL the implementations went for different approaches:
* On the F1 HAL the timer is initially stopped and you have to explicitly start it
* On the F3 HAL the constructor takes the timeout frequency as a parameter and returns an already running timer

[embedded-hal CountDown trait]: https://docs.rs/stm32f3xx-hal/0.4.3/stm32f3xx_hal/prelude/trait._embedded_hal_timer_CountDown.html
[f3 tim2]: https://docs.rs/stm32f3xx-hal/0.4.3/stm32f3xx_hal/timer/struct.Timer.html#method.tim2
[f3 spi example]: https://github.com/stm32-rs/stm32f3xx-hal/blob/master/examples/spi.rs#L24


### The bigger picture

Let's take a step back and look at the structure of the code we just wrote. It is quite a common pattern in the HAL world.

We found a struct for what we wanted to do, `Timer`. This struct has a bunch of methods along with "constructors" with the names `timX`. Each constructor takes a part of the `pac::Peripherals`, mutable references to some  registers (rcc) and, in the case of the f3, some configuration (the timer period).

This is how you would almost always instanciate peripherals in HALs, so look for that pattern when setting up the structs.

### What just happened

> You may also be interested in why we have to do this dance just to get the timer running. Couldn't there just be a method to start a timer directly on the `dp.TIM2` object? Why do we need to pass a bunch of registers to construrctors?
> 
> To answer the first question, we need to look at what the things in `dp` are. They are just a bunch of registers, in this case registers used for the timer peripheral. However, nothing is known about the state of the registers, or what they are used for currently. The timer registers can be used for, among other things, PWM, QEI, PWM input and of course, timers. Each application requires a different configuration
> 
> That is precisely what the `Timer` struct does. It sets up the registers to act as a regular timer. It then provides a bunch of methods for workin with the timer. The reason it takes ownership over `dp.TIMx` is to ensure that noone else uses that timer for something else. While the Timer exists, it is the sole owner and can assume a certain configuration.
> 
> What's with the other arguments (clocks and rcc) then? The `rcc` perheral is a central part of the CPU that controls the power and reset of other peripherals. To use a timer, it must to be turned on in the corresponding `rcc` register. The mutable reference ensures that noone else is writing to the register while we are, preventing race conditions.
>
> Finally, the `clocks` struct. Like `rcc`, this is another struct that is widely used. It contains the frequencies of all the clocks in the device. It existing implies that no clocks will change in the future, which guarantees that things like timers work the way they should.


<!--
I'll put a few of these comments in places where the guide might
need updates as the API of certain modules is changed. In this case,
I think it is likely that timers may change in the future
-->
<!-- UPDATE NOTE: Timer -->
