## Debug messages and cargo embed

Before we start jumping into some more advanced projects, it is worth looking at how to debug our projects.

When we were discussing panic handlers, we decided to use `panic_rtt`  because it allows us to get panic messages if panics occur. However, if you try to `panic!("test")` somewhere in your program, nothing will show that.

RTT is a mechanism that allows sending and receiving messages from our microcontroller through our debug probe but at the moment we have no way to read those messages. `cargo-embed` solves this issue, along with providing a bunch more convenience when working with our microcontroller.

Start by installing the tool
```
cargo install cargo-embed
```

Among other things, `cargo-embed` is a replacement for `cargo-flash` which uses a configuration file for specifying configuration instead of command line arguments. 

The tool is configured using an `Embed.toml` file. Create it, and put the following in it, replacing `your chip` with the argument you pass to `cargo flash`
```toml
[default.general]
chip = "your chip"
```

Now you can replace `cargo flash --chip ...` with `cargo embed` :). Hopefully this works. If you don't care about debugging messages, you can use whichever you prefer, but for rtt, you have to do `cargo embed`.

Let's get to the debugging part. To read rtt messages, you need to enable the RTT UI in `Embed.toml`

```toml
[default.rtt]
# Whether or not an RTTUI should be opened after flashing.
# This is exclusive and cannot be used with GDB at the moment.
enabled = true
```

Re-run `cargo embed` and you should get an interface that says "Terminal" at the top. If you add a `panic!` to your code, it should print that message in that terminal.

### `println` through rtt

Apart from just panic messages, `rtt` can also be used for printing messages, to do so, we need to add another crate

```toml
rtt-target = { version = "0.2.0", features = ["cortex-m"]}
```

Then, before we do anything else, we need to initialise  things using `rtt_init_print!`, and finally we can use `rprintln!` as a replacement for normal `println!`

```rust
use rtt_target::{rtt_init_print, rprintln};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    // Test a print
    rprintln!("Hello, world!");
    // Test a panic
    panic!("Panic");
}
```
